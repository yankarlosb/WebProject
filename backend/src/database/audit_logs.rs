//! `SeaORM` Entity. Generated manually for audit_logs table

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "audit_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub event_type: String,
    pub category: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<i32>,
    pub description: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::usuarios::Entity",
        from = "Column::UserId",
        to = "super::usuarios::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Usuarios,
}

impl Related<super::usuarios::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usuarios.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// ============================================================================
// TIPOS DE EVENTOS Y CATEGORÍAS
// ============================================================================

/// Tipos de eventos para auditoría
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Login,
    Logout,
    LoginFailed,
    Create,
    Update,
    Delete,
    Error,
    AccessDenied,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Login => "LOGIN",
            EventType::Logout => "LOGOUT",
            EventType::LoginFailed => "LOGIN_FAILED",
            EventType::Create => "CREATE",
            EventType::Update => "UPDATE",
            EventType::Delete => "DELETE",
            EventType::Error => "ERROR",
            EventType::AccessDenied => "ACCESS_DENIED",
        }
    }
}

/// Categorías de auditoría
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditCategory {
    Security,
    Functional,
}

impl AuditCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditCategory::Security => "SECURITY",
            AuditCategory::Functional => "FUNCTIONAL",
        }
    }
}

/// Tipos de entidades afectadas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    User,
    Subject,
    Balance,
    Session,
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::User => "USER",
            EntityType::Subject => "SUBJECT",
            EntityType::Balance => "BALANCE",
            EntityType::Session => "SESSION",
        }
    }
}
