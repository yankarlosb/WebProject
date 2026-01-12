//! Módulo de auditoría para registrar eventos del sistema
//! 
//! Este módulo proporciona funcionalidades para:
//! - Registrar eventos de seguridad (login, logout, accesos denegados)
//! - Registrar eventos funcionales (CRUD de entidades)
//! - Consultar trazas de auditoría

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::database::audit_logs::{self, ActiveModel, AuditCategory, EntityType, EventType};

/// Flag global para controlar si se registran IPs en los logs
pub static AUDIT_LOG_IP: AtomicBool = AtomicBool::new(true);

/// Actualiza la configuración de logging de IP
pub fn set_audit_log_ip(enabled: bool) {
    AUDIT_LOG_IP.store(enabled, Ordering::SeqCst);
}

/// Obtiene el estado actual del logging de IP
pub fn get_audit_log_ip() -> bool {
    AUDIT_LOG_IP.load(Ordering::SeqCst)
}

/// Estructura para crear un nuevo registro de auditoría
pub struct AuditLogBuilder {
    user_id: Option<i32>,
    user_name: Option<String>,
    event_type: EventType,
    category: AuditCategory,
    entity_type: Option<EntityType>,
    entity_id: Option<i32>,
    description: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    success: bool,
    error_message: Option<String>,
}

impl AuditLogBuilder {
    /// Crea un nuevo builder para un evento de auditoría
    pub fn new(event_type: EventType, category: AuditCategory, description: impl Into<String>) -> Self {
        Self {
            user_id: None,
            user_name: None,
            event_type,
            category,
            entity_type: None,
            entity_id: None,
            description: description.into(),
            ip_address: None,
            user_agent: None,
            success: true,
            error_message: None,
        }
    }

    /// Establece el usuario que realizó la acción
    pub fn user(mut self, user_id: i32, user_name: impl Into<String>) -> Self {
        self.user_id = Some(user_id);
        self.user_name = Some(user_name.into());
        self
    }

    /// Establece la entidad afectada
    pub fn entity(mut self, entity_type: EntityType, entity_id: i32) -> Self {
        self.entity_type = Some(entity_type);
        self.entity_id = Some(entity_id);
        self
    }

    /// Establece la dirección IP del cliente
    pub fn ip(mut self, ip_address: impl Into<String>) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }

    /// Establece el user agent del cliente
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Marca el evento como fallido
    pub fn failed(mut self, error_message: impl Into<String>) -> Self {
        self.success = false;
        self.error_message = Some(error_message.into());
        self
    }

    /// Guarda el registro de auditoría en la base de datos
    pub async fn save(self, db: &DatabaseConnection) -> Result<audit_logs::Model, sea_orm::DbErr> {
        // Respetar la configuración de audit_log_ip
        let ip_address = if get_audit_log_ip() {
            self.ip_address
        } else {
            None
        };
        
        let log = ActiveModel {
            user_id: Set(self.user_id),
            user_name: Set(self.user_name),
            event_type: Set(self.event_type.as_str().to_string()),
            category: Set(self.category.as_str().to_string()),
            entity_type: Set(self.entity_type.map(|e| e.as_str().to_string())),
            entity_id: Set(self.entity_id),
            description: Set(self.description),
            ip_address: Set(ip_address),
            user_agent: Set(self.user_agent),
            success: Set(self.success),
            error_message: Set(self.error_message),
            ..Default::default()
        };

        log.insert(db).await
    }
}

// ============================================================================
// FUNCIONES DE CONVENIENCIA PARA EVENTOS COMUNES
// ============================================================================

/// Registra un inicio de sesión exitoso
pub async fn log_login_success(
    db: &DatabaseConnection,
    user_id: i32,
    user_name: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::Login,
        AuditCategory::Functional,
        format!("Usuario '{}' inició sesión exitosamente", user_name),
    )
    .user(user_id, user_name)
    .entity(EntityType::Session, user_id)
    .ip(ip_address)
    .save(db)
    .await?;
    Ok(())
}

/// Registra un intento de inicio de sesión fallido
pub async fn log_login_failed(
    db: &DatabaseConnection,
    user_name: &str,
    ip_address: &str,
    reason: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::LoginFailed,
        AuditCategory::Security,
        format!("Intento de inicio de sesión fallido para usuario '{}'", user_name),
    )
    .user_agent(user_name) // No tenemos ID porque falló
    .ip(ip_address)
    .failed(reason)
    .save(db)
    .await?;
    Ok(())
}

/// Registra un cierre de sesión
pub async fn log_logout(
    db: &DatabaseConnection,
    user_id: i32,
    user_name: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::Logout,
        AuditCategory::Functional,
        format!("Usuario '{}' cerró sesión", user_name),
    )
    .user(user_id, user_name)
    .entity(EntityType::Session, user_id)
    .ip(ip_address)
    .save(db)
    .await?;
    Ok(())
}

/// Registra la creación de un usuario
pub async fn log_user_created(
    db: &DatabaseConnection,
    admin_id: i32,
    admin_name: &str,
    new_user_id: i32,
    new_user_name: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::Create,
        AuditCategory::Security,
        format!("Admin '{}' creó el usuario '{}'", admin_name, new_user_name),
    )
    .user(admin_id, admin_name)
    .entity(EntityType::User, new_user_id)
    .ip(ip_address)
    .save(db)
    .await?;
    Ok(())
}

/// Registra la modificación de un usuario
pub async fn log_user_modified(
    db: &DatabaseConnection,
    admin_id: i32,
    admin_name: &str,
    modified_user_id: i32,
    modified_user_name: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::Update,
        AuditCategory::Security,
        format!("Admin '{}' modificó el usuario '{}'", admin_name, modified_user_name),
    )
    .user(admin_id, admin_name)
    .entity(EntityType::User, modified_user_id)
    .ip(ip_address)
    .save(db)
    .await?;
    Ok(())
}

/// Registra la eliminación de un usuario
pub async fn log_user_deleted(
    db: &DatabaseConnection,
    admin_id: i32,
    admin_name: &str,
    deleted_user_id: i32,
    deleted_user_name: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::Delete,
        AuditCategory::Security,
        format!("Admin '{}' eliminó el usuario '{}' (ID: {})", admin_name, deleted_user_name, deleted_user_id),
    )
    .user(admin_id, admin_name)
    .entity(EntityType::User, deleted_user_id)
    .ip(ip_address)
    .save(db)
    .await?;
    Ok(())
}

/// Registra la creación de una asignatura
pub async fn log_subject_created(
    db: &DatabaseConnection,
    leader_id: i32,
    leader_name: &str,
    subject_id: i32,
    subject_name: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::Create,
        AuditCategory::Functional,
        format!("Leader '{}' creó la asignatura '{}'", leader_name, subject_name),
    )
    .user(leader_id, leader_name)
    .entity(EntityType::Subject, subject_id)
    .ip(ip_address)
    .save(db)
    .await?;
    Ok(())
}

/// Registra la modificación de una asignatura
pub async fn log_subject_modified(
    db: &DatabaseConnection,
    user_id: i32,
    user_name: &str,
    subject_id: i32,
    subject_name: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::Update,
        AuditCategory::Functional,
        format!("Usuario '{}' modificó la asignatura '{}'", user_name, subject_name),
    )
    .user(user_id, user_name)
    .entity(EntityType::Subject, subject_id)
    .ip(ip_address)
    .save(db)
    .await?;
    Ok(())
}

/// Registra la eliminación de una asignatura
pub async fn log_subject_deleted(
    db: &DatabaseConnection,
    leader_id: i32,
    leader_name: &str,
    subject_id: i32,
    subject_name: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    AuditLogBuilder::new(
        EventType::Delete,
        AuditCategory::Functional,
        format!("Leader '{}' eliminó la asignatura '{}' (ID: {})", leader_name, subject_name, subject_id),
    )
    .user(leader_id, leader_name)
    .entity(EntityType::Subject, subject_id)
    .ip(ip_address)
    .save(db)
    .await?;
    Ok(())
}

/// Registra un intento de acceso denegado
pub async fn log_access_denied(
    db: &DatabaseConnection,
    user_id: Option<i32>,
    user_name: Option<&str>,
    resource: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    let mut builder = AuditLogBuilder::new(
        EventType::AccessDenied,
        AuditCategory::Security,
        format!("Acceso denegado a recurso: {}", resource),
    )
    .ip(ip_address)
    .failed("Permisos insuficientes");

    if let (Some(id), Some(name)) = (user_id, user_name) {
        builder = builder.user(id, name);
    }

    builder.save(db).await?;
    Ok(())
}

/// Registra un error del sistema
pub async fn log_error(
    db: &DatabaseConnection,
    user_id: Option<i32>,
    user_name: Option<&str>,
    error_description: &str,
    ip_address: &str,
) -> Result<(), sea_orm::DbErr> {
    let mut builder = AuditLogBuilder::new(
        EventType::Error,
        AuditCategory::Security,
        error_description.to_string(),
    )
    .ip(ip_address)
    .failed(error_description);

    if let (Some(id), Some(name)) = (user_id, user_name) {
        builder = builder.user(id, name);
    }

    builder.save(db).await?;
    Ok(())
}

// ============================================================================
// FUNCIONES DE CONSULTA
// ============================================================================

/// Obtiene los logs de auditoría más recientes
pub async fn get_recent_logs(
    db: &DatabaseConnection,
    limit: u64,
) -> Result<Vec<audit_logs::Model>, sea_orm::DbErr> {
    audit_logs::Entity::find()
        .order_by_desc(audit_logs::Column::CreatedAt)
        .limit(limit)
        .all(db)
        .await
}

/// Obtiene logs filtrados por categoría
pub async fn get_logs_by_category(
    db: &DatabaseConnection,
    category: AuditCategory,
    limit: u64,
) -> Result<Vec<audit_logs::Model>, sea_orm::DbErr> {
    audit_logs::Entity::find()
        .filter(audit_logs::Column::Category.eq(category.as_str()))
        .order_by_desc(audit_logs::Column::CreatedAt)
        .limit(limit)
        .all(db)
        .await
}

/// Obtiene logs filtrados por tipo de evento
pub async fn get_logs_by_event_type(
    db: &DatabaseConnection,
    event_type: EventType,
    limit: u64,
) -> Result<Vec<audit_logs::Model>, sea_orm::DbErr> {
    audit_logs::Entity::find()
        .filter(audit_logs::Column::EventType.eq(event_type.as_str()))
        .order_by_desc(audit_logs::Column::CreatedAt)
        .limit(limit)
        .all(db)
        .await
}

/// Obtiene logs de un usuario específico
pub async fn get_logs_by_user(
    db: &DatabaseConnection,
    user_id: i32,
    limit: u64,
) -> Result<Vec<audit_logs::Model>, sea_orm::DbErr> {
    audit_logs::Entity::find()
        .filter(audit_logs::Column::UserId.eq(user_id))
        .order_by_desc(audit_logs::Column::CreatedAt)
        .limit(limit)
        .all(db)
        .await
}

/// Cuenta el total de logs
pub async fn count_logs(db: &DatabaseConnection) -> Result<u64, sea_orm::DbErr> {
    use sea_orm::PaginatorTrait;
    audit_logs::Entity::find().count(db).await
}

/// Cuenta logs por tipo de evento (más eficiente que obtener todos y contar)
pub async fn count_logs_by_event_type(
    db: &DatabaseConnection,
    event_type: EventType,
) -> Result<u64, sea_orm::DbErr> {
    use sea_orm::PaginatorTrait;
    audit_logs::Entity::find()
        .filter(audit_logs::Column::EventType.eq(event_type.as_str()))
        .count(db)
        .await
}

/// Obtiene solo los logs de seguridad (para el panel de admin)
pub async fn get_security_logs(
    db: &DatabaseConnection,
    limit: u64,
) -> Result<Vec<audit_logs::Model>, sea_orm::DbErr> {
    get_logs_by_category(db, AuditCategory::Security, limit).await
}

/// Elimina logs de auditoría más antiguos que el número de días especificado
/// Retorna el número de logs eliminados
pub async fn cleanup_old_logs(
    db: &DatabaseConnection,
    retention_days: i32,
) -> Result<u64, sea_orm::DbErr> {
    use chrono::{Duration, Utc};
    
    let cutoff_date = Utc::now() - Duration::days(retention_days as i64);
    
    let result = audit_logs::Entity::delete_many()
        .filter(audit_logs::Column::CreatedAt.lt(cutoff_date.naive_utc()))
        .exec(db)
        .await?;
    
    Ok(result.rows_affected)
}
