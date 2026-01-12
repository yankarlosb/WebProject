//! Settings API Routes
//!
//! Provides endpoints for managing system settings (admin only).
//! Settings are stored in the database and can be modified at runtime.

use rocket::serde::json::Json;
use rocket::{get, put, State};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::database::audit_logs::{AuditCategory, EventType};
use crate::database::system_settings;
use crate::types::{ApiResponse, ApiResponseWithData};
use crate::utils::audit::AuditLogBuilder;
use crate::utils::jwt::{AdminUser, set_ip_validation};
use crate::AppState;

/// Response structure for a single setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingResponse {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
    pub category: String,
}

/// Response structure for settings grouped by category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsGrouped {
    pub security: Vec<SettingResponse>,
    pub session: Vec<SettingResponse>,
    pub password: Vec<SettingResponse>,
    pub audit: Vec<SettingResponse>,
}

/// Request structure for updating settings
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: HashMap<String, String>,
}

/// Response structure for public settings (session timeout)
#[derive(Debug, Clone, Serialize)]
pub struct PublicSettingsResponse {
    pub session_timeout_minutes: i32,
}

/// GET /api/settings/public - Get public settings (session timeout, etc.)
/// This endpoint is accessible to authenticated users (not admin-only)
#[get("/settings/public")]
pub async fn get_public_settings(
    db: &State<AppState>,
    _user: crate::utils::jwt::AuthenticatedUser,
) -> Json<ApiResponseWithData<PublicSettingsResponse>> {
    let session_timeout = get_setting_i32(&db.db, "session_timeout_minutes", 30).await;
    
    Json(ApiResponseWithData {
        message: "Configuraciones públicas obtenidas".to_string(),
        alert: "success".to_string(),
        data: Some(PublicSettingsResponse {
            session_timeout_minutes: session_timeout,
        }),
    })
}

/// GET /api/settings - List all system settings (Admin only)
#[get("/settings")]
pub async fn list_settings(
    db: &State<AppState>,
    admin: AdminUser,
) -> Json<ApiResponseWithData<SettingsGrouped>> {
    let _ = admin; // Ensure admin access

    match system_settings::Entity::find()
        .order_by_asc(system_settings::Column::Category)
        .order_by_asc(system_settings::Column::Key)
        .all(&db.db)
        .await
    {
        Ok(settings) => {
            let mut grouped = SettingsGrouped {
                security: Vec::new(),
                session: Vec::new(),
                password: Vec::new(),
                audit: Vec::new(),
            };

            for setting in settings {
                let response = SettingResponse {
                    key: setting.key,
                    value: setting.value,
                    description: setting.description,
                    category: setting.category.clone(),
                };

                match setting.category.as_str() {
                    "security" => grouped.security.push(response),
                    "session" => grouped.session.push(response),
                    "password" => grouped.password.push(response),
                    "audit" => grouped.audit.push(response),
                    _ => {} // Ignore unknown categories
                }
            }

            Json(ApiResponseWithData {
                message: "Configuraciones obtenidas".to_string(),
                alert: "success".to_string(),
                data: Some(grouped),
            })
        }
        Err(e) => Json(ApiResponseWithData {
            message: format!("Error al obtener configuraciones: {}", e),
            alert: "error".to_string(),
            data: None,
        }),
    }
}

/// PUT /api/settings - Update system settings (Admin only)
#[put("/settings", data = "<request>")]
pub async fn update_settings(
    db: &State<AppState>,
    admin: AdminUser,
    request: Json<UpdateSettingsRequest>,
) -> Json<ApiResponse> {
    let updated_keys: Vec<String> = request.settings.keys().cloned().collect();

    // Update each setting
    for (key, value) in &request.settings {
        // Find existing setting
        match system_settings::Entity::find_by_id(key.clone())
            .one(&db.db)
            .await
        {
            Ok(Some(setting)) => {
                // Update the setting
                let mut active: system_settings::ActiveModel = setting.into();
                active.value = Set(value.clone());

                if let Err(e) = active.update(&db.db).await {
                    // Log failure
                    let _ = AuditLogBuilder::new(
                        EventType::SettingsUpdated,
                        AuditCategory::Security,
                        format!("Error al actualizar configuración '{}': {}", key, e),
                    )
                    .user(admin.0.sub.parse().unwrap_or(0), &admin.0.user_name)
                    .ip(&admin.0.ip)
                    .failed(&e.to_string())
                    .save(&db.db)
                    .await;

                    return Json(ApiResponse {
                        message: format!("Error al actualizar '{}': {}", key, e),
                        alert: "error".to_string(),
                    });
                }
            }
            Ok(None) => {
                return Json(ApiResponse {
                    message: format!("Configuración '{}' no encontrada", key),
                    alert: "error".to_string(),
                });
            }
            Err(e) => {
                return Json(ApiResponse {
                    message: format!("Error al buscar '{}': {}", key, e),
                    alert: "error".to_string(),
                });
            }
        }
    }

    // Log success
    let _ = AuditLogBuilder::new(
        EventType::SettingsUpdated,
        AuditCategory::Security,
        format!("Admin '{}' actualizó configuraciones: {:?}", admin.0.user_name, updated_keys),
    )
    .user(admin.0.sub.parse().unwrap_or(0), &admin.0.user_name)
    .ip(&admin.0.ip)
    .save(&db.db)
    .await;

    // Aplicar configuraciones que necesitan actualización inmediata
    if updated_keys.contains(&"require_ip_validation".to_string()) {
        let require_ip = get_setting_bool(&db.db, "require_ip_validation", true).await;
        set_ip_validation(require_ip);
    }

    // Actualizar rate limiter si se modificaron sus configuraciones
    let rate_limiter_keys = ["max_login_attempts", "block_duration_minutes", "attempt_window_minutes"];
    if updated_keys.iter().any(|k| rate_limiter_keys.contains(&k.as_str())) {
        let config = load_rate_limiter_config(&db.db).await;
        db.rate_limiter.update_config(config);
    }

    // Actualizar política de contraseñas si se modificaron sus configuraciones
    let password_policy_keys = ["password_min_length", "password_require_uppercase", "password_require_lowercase", "password_require_special"];
    if updated_keys.iter().any(|k| password_policy_keys.contains(&k.as_str())) {
        use crate::utils::validation::set_password_policy;
        let policy = load_password_policy(&db.db).await;
        set_password_policy(policy);
    }

    // Actualizar configuración de logging de IP en auditoría
    if updated_keys.contains(&"audit_log_ip".to_string()) {
        use crate::utils::audit::set_audit_log_ip;
        let audit_log_ip = get_setting_bool(&db.db, "audit_log_ip", true).await;
        set_audit_log_ip(audit_log_ip);
    }

    Json(ApiResponse {
        message: "Configuraciones actualizadas correctamente".to_string(),
        alert: "success".to_string(),
    })
}

/// Get a single setting value from the database
/// Returns the default value if the setting doesn't exist
pub async fn get_setting(db: &DatabaseConnection, key: &str, default: &str) -> String {
    match system_settings::Entity::find_by_id(key.to_string())
        .one(db)
        .await
    {
        Ok(Some(setting)) => setting.value,
        _ => default.to_string(),
    }
}

/// Get a numeric setting value from the database
pub async fn get_setting_i32(db: &DatabaseConnection, key: &str, default: i32) -> i32 {
    get_setting(db, key, &default.to_string())
        .await
        .parse()
        .unwrap_or(default)
}

/// Get a boolean setting value from the database
pub async fn get_setting_bool(db: &DatabaseConnection, key: &str, default: bool) -> bool {
    let value = get_setting(db, key, if default { "true" } else { "false" }).await;
    value.to_lowercase() == "true" || value == "1"
}

/// Get a u64 setting value from the database
pub async fn get_setting_u64(db: &DatabaseConnection, key: &str, default: u64) -> u64 {
    get_setting(db, key, &default.to_string())
        .await
        .parse()
        .unwrap_or(default)
}

/// Load rate limiter configuration from database
pub async fn load_rate_limiter_config(db: &DatabaseConnection) -> crate::utils::rate_limiter::RateLimiterConfig {
    use crate::utils::rate_limiter::{RateLimiterConfig, DEFAULT_MAX_ATTEMPTS, DEFAULT_BLOCK_DURATION_SECS, DEFAULT_ATTEMPT_WINDOW_SECS};
    
    let max_attempts = get_setting_i32(db, "max_login_attempts", DEFAULT_MAX_ATTEMPTS as i32).await as u32;
    let block_duration_mins = get_setting_i32(db, "block_duration_minutes", (DEFAULT_BLOCK_DURATION_SECS / 60) as i32).await;
    let attempt_window_mins = get_setting_i32(db, "attempt_window_minutes", (DEFAULT_ATTEMPT_WINDOW_SECS / 60) as i32).await;
    
    RateLimiterConfig {
        max_attempts,
        block_duration_secs: (block_duration_mins * 60) as u64,
        attempt_window_secs: (attempt_window_mins * 60) as u64,
    }
}

/// Load password policy from database
pub async fn load_password_policy(db: &DatabaseConnection) -> crate::utils::validation::PasswordPolicy {
    use crate::utils::validation::PasswordPolicy;
    
    PasswordPolicy {
        min_length: get_setting_i32(db, "password_min_length", 8).await as usize,
        require_uppercase: get_setting_bool(db, "password_require_uppercase", true).await,
        require_lowercase: get_setting_bool(db, "password_require_lowercase", true).await,
        require_special: get_setting_bool(db, "password_require_special", true).await,
    }
}

/// Load IP validation setting from database
pub async fn load_ip_validation_setting(db: &DatabaseConnection) -> bool {
    get_setting_bool(db, "require_ip_validation", true).await
}

/// Load audit log IP setting from database
pub async fn load_audit_log_ip_setting(db: &DatabaseConnection) -> bool {
    get_setting_bool(db, "audit_log_ip", true).await
}
