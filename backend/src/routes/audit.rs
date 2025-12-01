//! Rutas de API para el sistema de auditoría
//! Solo accesibles por administradores

use crate::utils::jwt::AdminUser;
use crate::types::ApiResponseWithData;
use crate::database::audit_logs;
use crate::*;
use rocket::get;
use serde::{Deserialize, Serialize};

/// Estructura de respuesta para logs de auditoría (serialización para frontend)
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogResponse {
    pub id: i32,
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub event_type: String,
    pub category: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<i32>,
    pub description: String,
    pub ip_address: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: Option<String>,
}

impl From<audit_logs::Model> for AuditLogResponse {
    fn from(log: audit_logs::Model) -> Self {
        Self {
            id: log.id,
            user_id: log.user_id,
            user_name: log.user_name,
            event_type: log.event_type,
            category: log.category,
            entity_type: log.entity_type,
            entity_id: log.entity_id,
            description: log.description,
            ip_address: log.ip_address,
            success: log.success,
            error_message: log.error_message,
            created_at: log.created_at.map(|dt| dt.to_string()),
        }
    }
}

/// Listar logs de auditoría (últimos 100)
#[get("/audit/logs")]
pub async fn list_audit_logs(
    db: &State<AppState>,
    _admin: AdminUser,
) -> Json<ApiResponseWithData<Vec<AuditLogResponse>>> {
    match utils::audit::get_recent_logs(&db.db, 100).await {
        Ok(logs) => {
            let response_logs: Vec<AuditLogResponse> = logs.into_iter().map(|l| l.into()).collect();
            Json(ApiResponseWithData::success(
                "Logs de auditoría obtenidos exitosamente".to_string(),
                response_logs,
            ))
        }
        Err(e) => Json(ApiResponseWithData::error(format!(
            "Error al obtener logs de auditoría: {}",
            e
        ))),
    }
}

/// Listar solo logs de seguridad
#[get("/audit/logs/security")]
pub async fn list_security_logs(
    db: &State<AppState>,
    _admin: AdminUser,
) -> Json<ApiResponseWithData<Vec<AuditLogResponse>>> {
    match utils::audit::get_security_logs(&db.db, 50).await {
        Ok(logs) => {
            let response_logs: Vec<AuditLogResponse> = logs.into_iter().map(|l| l.into()).collect();
            Json(ApiResponseWithData::success(
                "Logs de seguridad obtenidos exitosamente".to_string(),
                response_logs,
            ))
        }
        Err(e) => Json(ApiResponseWithData::error(format!(
            "Error al obtener logs de seguridad: {}",
            e
        ))),
    }
}

/// Estadísticas de auditoría
#[derive(Debug, Serialize)]
pub struct AuditStats {
    pub total_logs: u64,
    pub recent_logins: u64,
    pub recent_errors: u64,
}

#[get("/audit/stats")]
pub async fn get_audit_stats(
    db: &State<AppState>,
    _admin: AdminUser,
) -> Json<ApiResponseWithData<AuditStats>> {
    let total = utils::audit::count_logs(&db.db).await.unwrap_or(0);
    
    // Contar logins recientes
    let logins = utils::audit::get_logs_by_event_type(
        &db.db,
        audit_logs::EventType::Login,
        100,
    )
    .await
    .map(|v| v.len() as u64)
    .unwrap_or(0);
    
    // Contar errores recientes
    let errors = utils::audit::get_logs_by_event_type(
        &db.db,
        audit_logs::EventType::Error,
        100,
    )
    .await
    .map(|v| v.len() as u64)
    .unwrap_or(0);

    let stats = AuditStats {
        total_logs: total,
        recent_logins: logins,
        recent_errors: errors,
    };

    Json(ApiResponseWithData::success(
        "Estadísticas de auditoría obtenidas".to_string(),
        stats,
    ))
}
