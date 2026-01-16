//! Rutas de API para el sistema de auditoría
//! Solo accesibles por administradores

use crate::utils::jwt::AdminUser;
use crate::types::ApiResponseWithData;
use crate::database::audit_logs;
use crate::*;
use rocket::{get, post};
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
    // Use parallel queries for better performance
    let (total, logins, errors) = tokio::join!(
        utils::audit::count_logs(&db.db),
        utils::audit::count_logs_by_event_type(&db.db, audit_logs::EventType::Login),
        utils::audit::count_logs_by_event_type(&db.db, audit_logs::EventType::Error)
    );

    let stats = AuditStats {
        total_logs: total.unwrap_or(0),
        recent_logins: logins.unwrap_or(0),
        recent_errors: errors.unwrap_or(0),
    };

    Json(ApiResponseWithData::success(
        "Estadísticas de auditoría obtenidas".to_string(),
        stats,
    ))
}

/// Respuesta para la limpieza de logs
#[derive(Debug, Serialize)]
pub struct CleanupResponse {
    pub deleted_count: u64,
}

/// POST /api/audit/cleanup - Limpiar logs antiguos según la configuración de retención
#[post("/audit/cleanup")]
pub async fn cleanup_audit_logs(
    db: &State<AppState>,
    admin: AdminUser,
) -> Json<ApiResponseWithData<CleanupResponse>> {
    // Obtener días de retención de la configuración, forzando mínimo 90 días por seguridad
    let retention_days = crate::routes::settings::get_setting_i32(
        &db.db, 
        "audit_retention_days", 
        90
    ).await.max(90);
    
    match utils::audit::cleanup_old_logs(&db.db, retention_days).await {
        Ok(deleted_count) => {
            // Registrar la acción de limpieza
            let _ = crate::utils::audit::AuditLogBuilder::new(
                audit_logs::EventType::Delete,
                audit_logs::AuditCategory::Functional,
                format!(
                    "Admin '{}' ejecutó limpieza de logs: {} registros eliminados (retención: {} días)",
                    admin.0.user_name, deleted_count, retention_days
                ),
            )
            .user(admin.0.sub.parse().unwrap_or(0), &admin.0.user_name)
            .ip(&admin.0.ip)
            .save(&db.db)
            .await;
            
            Json(ApiResponseWithData::success(
                format!("Se eliminaron {} logs antiguos (más de {} días)", deleted_count, retention_days),
                CleanupResponse { deleted_count },
            ))
        }
        Err(e) => Json(ApiResponseWithData::error(format!(
            "Error al limpiar logs antiguos: {}",
            e
        ))),
    }
}

/// Respuesta personalizada para descarga de archivos
pub struct LogFileResponse {
    content: String,
    filename: String,
}

impl<'r> rocket::response::Responder<'r, 'static> for LogFileResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::response::Response::build()
            .header(rocket::http::ContentType::Plain)
            .header(rocket::http::Header::new("Content-Disposition", format!("attachment; filename=\"{}\"", self.filename)))
            .sized_body(self.content.len(), std::io::Cursor::new(self.content))
            .ok()
    }
}

/// Exportar logs de auditoría como archivo .log
#[get("/audit/export")]
pub async fn export_audit_logs(
    db: &State<AppState>,
    _admin: AdminUser,
) -> Result<LogFileResponse, String> {
    // Obtener todos los logs usando la función recién creada
    let logs = match utils::audit::get_all_logs(&db.db).await {
        Ok(logs) => logs,
        Err(e) => return Err(format!("Error al obtener logs: {}", e)),
    };

    // Crear el contenido del archivo
    let mut content = String::new();
    content.push_str("================================================================================\n");
    content.push_str("                        REPORTE DE AUDITORÍA DEL SISTEMA                        \n");
    content.push_str("================================================================================\n\n");
    
    content.push_str(&format!("Fecha de generación: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    content.push_str(&format!("Total de registros: {}\n\n", logs.len()));
    
    content.push_str("ID     | FECHA                      | USUARIO         | EVENTO               | CATEGORÍA  | ENTIDAD         | ESTADO | IP              | DESCRIPCIÓN\n");
    content.push_str("-----------------------------------------------------------------------------------------------------------------------------------------------------------------\n");

    for log in logs {

        let created_at = log.created_at.map(|d| d.to_string()).unwrap_or_else(|| "N/A".to_string());
        let user = log.user_name.as_deref().unwrap_or("Sistema");
        
        let entity = format!("{}:{}", log.entity_type.as_deref().unwrap_or("-"), log.entity_id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string()));
        
        let ip = log.ip_address.as_deref().unwrap_or("-");
        let status = if log.success { "EXITO" } else { "FALLO" };
        
        // Manejar descripción con saltos de línea para que no rompa el formato
        let desc = log.description.replace("\n", " ");

        content.push_str(&format!(
            "{:<6} | {:<26} | {:<15} | {:<20} | {:<10} | {:<15} | {:<6} | {:<15} | {}\n",
            log.id, created_at, user, log.event_type, log.category, entity, status, ip, desc
        ));
    }

    let filename = format!("audit_logs_{}.log", chrono::Local::now().format("%Y%m%d_%H%M%S"));

    Ok(LogFileResponse {
        content,
        filename,
    })
}
