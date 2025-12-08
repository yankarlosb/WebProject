//! Rutas CRUD para Balances de Carga Docente
//! 
//! Los balances se almacenan con sus asignaturas y valores en formato JSONB

use crate::utils::jwt::AuthenticatedUser;
use crate::utils::audit;
use crate::database::audit_logs::{EventType, AuditCategory, EntityType};
use crate::*;
use crate::types::{ApiResponse, ApiResponseWithData};
use crate::database::balance;
use rocket::{post, get, put, delete};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::net::SocketAddr;

/// Estructura para crear/actualizar un balance
#[derive(Debug, Deserialize)]
pub struct BalanceRequest {
    pub academic_year: String,        // '1ro', '2do', '3ro', '4to'
    pub period: String,               // '1ero', '2do'
    pub academic_year_text: String,   // '2025-2026'
    pub start_date: String,           // 'YYYY-MM-DD'
    pub weeks: i32,                   // Número de semanas
    pub subjects: serde_json::Value,  // Array JSON de asignaturas
}

/// Genera el nombre del balance automáticamente
fn generate_balance_name(academic_year: &str, period: &str, academic_year_text: &str) -> String {
    format!("{} Año - Período {} ({})", academic_year, period, academic_year_text)
}

/// Estructura para respuesta de balance (incluye ID)
#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub academic_year: String,
    pub period: String,
    pub academic_year_text: String,
    pub start_date: String,
    pub weeks: i32,
    pub subjects: serde_json::Value,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<balance::Model> for BalanceResponse {
    fn from(model: balance::Model) -> Self {
        BalanceResponse {
            id: model.id,
            user_id: model.user_id,
            name: model.name,
            academic_year: model.academic_year,
            period: model.period,
            academic_year_text: model.academic_year_text,
            start_date: model.start_date.to_string(),
            weeks: model.weeks,
            subjects: model.subjects,
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
        }
    }
}

/// Listar todos los balances (visibles para todos los usuarios autenticados)
#[get("/balances")]
pub async fn list_balances(
    db: &State<AppState>,
    _user: AuthenticatedUser,
) -> Json<ApiResponseWithData<Vec<BalanceResponse>>> {
    let result = balance::Entity::find()
        .order_by_desc(balance::Column::CreatedAt)
        .all(&db.db)
        .await;

    match result {
        Ok(balances) => {
            let responses: Vec<BalanceResponse> = balances.into_iter().map(Into::into).collect();
            Json(ApiResponseWithData::success(
                "Balances obtenidos exitosamente".to_string(),
                responses,
            ))
        }
        Err(e) => Json(ApiResponseWithData::error(format!(
            "Error al obtener balances: {}",
            e
        ))),
    }
}

/// Obtener un balance específico por ID (visible para todos los usuarios autenticados)
#[get("/balances/<balance_id>")]
pub async fn get_balance(
    balance_id: i32,
    db: &State<AppState>,
    _user: AuthenticatedUser,
) -> Json<ApiResponseWithData<BalanceResponse>> {
    let result = balance::Entity::find_by_id(balance_id)
        .one(&db.db)
        .await;

    match result {
        Ok(Some(balance_model)) => Json(ApiResponseWithData::success(
            "Balance obtenido exitosamente".to_string(),
            balance_model.into(),
        )),
        Ok(None) => Json(ApiResponseWithData::error(
            "Balance no encontrado".to_string(),
        )),
        Err(e) => Json(ApiResponseWithData::error(format!(
            "Error al obtener el balance: {}",
            e
        ))),
    }
}

/// Crear un nuevo balance
#[post("/balances", format = "json", data = "<balance_data>")]
pub async fn create_balance(
    balance_data: Json<BalanceRequest>,
    db: &State<AppState>,
    user: AuthenticatedUser,
    remote_addr: Option<SocketAddr>,
) -> Json<ApiResponseWithData<BalanceResponse>> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let data = balance_data.into_inner();

    // Parsear la fecha
    let start_date = match NaiveDate::parse_from_str(&data.start_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return Json(ApiResponseWithData::error(
                "Formato de fecha inválido. Use YYYY-MM-DD".to_string(),
            ))
        }
    };

    // Generar nombre automáticamente
    let name = generate_balance_name(&data.academic_year, &data.period, &data.academic_year_text);

    let new_balance = balance::ActiveModel {
        user_id: Set(user_id),
        name: Set(name.clone()),
        academic_year: Set(data.academic_year),
        period: Set(data.period),
        academic_year_text: Set(data.academic_year_text),
        start_date: Set(start_date),
        weeks: Set(data.weeks),
        subjects: Set(data.subjects),
        ..Default::default()
    };

    let ip_str = remote_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());

    match new_balance.insert(&db.db).await {
        Ok(inserted) => {
            // Registrar en auditoría
            let _ = audit::AuditLogBuilder::new(
                EventType::Create,
                AuditCategory::Functional,
                format!("Usuario '{}' creó el balance '{}'", user.0.user_name, name),
            )
            .user(user_id, &user.0.user_name)
            .entity(EntityType::Balance, inserted.id)
            .ip(&ip_str)
            .save(&db.db)
            .await;
            
            Json(ApiResponseWithData::success(
                "Balance creado exitosamente".to_string(),
                inserted.into(),
            ))
        },
        Err(e) => Json(ApiResponseWithData::error(format!(
            "Error al crear el balance: {}",
            e
        ))),
    }
}

/// Actualizar un balance existente
#[put("/balances/<balance_id>", format = "json", data = "<balance_data>")]
pub async fn update_balance(
    balance_id: i32,
    balance_data: Json<BalanceRequest>,
    db: &State<AppState>,
    user: AuthenticatedUser,
    remote_addr: Option<SocketAddr>,
) -> Json<ApiResponseWithData<BalanceResponse>> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let data = balance_data.into_inner();

    // Verificar que el balance pertenece al usuario
    let existing = balance::Entity::find_by_id(balance_id)
        .filter(balance::Column::UserId.eq(user_id))
        .one(&db.db)
        .await;

    let ip_str = remote_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());

    match existing {
        Ok(Some(balance_model)) => {
            // Parsear la fecha
            let start_date = match NaiveDate::parse_from_str(&data.start_date, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    return Json(ApiResponseWithData::error(
                        "Formato de fecha inválido. Use YYYY-MM-DD".to_string(),
                    ))
                }
            };

            // Generar nombre automáticamente
            let name = generate_balance_name(&data.academic_year, &data.period, &data.academic_year_text);

            let mut active_model: balance::ActiveModel = balance_model.into();
            active_model.name = Set(name.clone());
            active_model.academic_year = Set(data.academic_year);
            active_model.period = Set(data.period);
            active_model.academic_year_text = Set(data.academic_year_text);
            active_model.start_date = Set(start_date);
            active_model.weeks = Set(data.weeks);
            active_model.subjects = Set(data.subjects);
            active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

            match active_model.update(&db.db).await {
                Ok(updated) => {
                    // Registrar en auditoría
                    let _ = audit::AuditLogBuilder::new(
                        EventType::Update,
                        AuditCategory::Functional,
                        format!("Usuario '{}' actualizó el balance '{}'", user.0.user_name, name),
                    )
                    .user(user_id, &user.0.user_name)
                    .entity(EntityType::Balance, balance_id)
                    .ip(&ip_str)
                    .save(&db.db)
                    .await;
                    
                    Json(ApiResponseWithData::success(
                        "Balance actualizado exitosamente".to_string(),
                        updated.into(),
                    ))
                },
                Err(e) => Json(ApiResponseWithData::error(format!(
                    "Error al actualizar el balance: {}",
                    e
                ))),
            }
        }
        Ok(None) => Json(ApiResponseWithData::error(
            "Balance no encontrado o no tienes permiso para editarlo".to_string(),
        )),
        Err(e) => Json(ApiResponseWithData::error(format!(
            "Error al buscar el balance: {}",
            e
        ))),
    }
}

/// Eliminar un balance
#[delete("/balances/<balance_id>")]
pub async fn delete_balance(
    balance_id: i32,
    db: &State<AppState>,
    user: AuthenticatedUser,
    remote_addr: Option<SocketAddr>,
) -> Json<ApiResponse> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let ip_str = remote_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());

    // Find and delete in one query - get the name for logging and delete in a single db roundtrip
    let balance_to_delete = match balance::Entity::find_by_id(balance_id)
        .filter(balance::Column::UserId.eq(user_id))
        .one(&db.db)
        .await
    {
        Ok(Some(b)) => b,
        Ok(None) => {
            return Json(ApiResponse::error(
                "Balance no encontrado o no tienes permiso para eliminarlo".to_string(),
            ))
        }
        Err(e) => {
            return Json(ApiResponse::error(format!(
                "Error al buscar el balance: {}",
                e
            )))
        }
    };

    let balance_name = balance_to_delete.name.clone();

    // Delete the found balance using ModelTrait
    match balance_to_delete.delete(&db.db).await {
        Ok(_) => {
            // Registrar en auditoría
            let _ = audit::AuditLogBuilder::new(
                EventType::Delete,
                AuditCategory::Functional,
                format!("Usuario '{}' eliminó el balance '{}' (ID: {})", user.0.user_name, balance_name, balance_id),
            )
            .user(user_id, &user.0.user_name)
            .entity(EntityType::Balance, balance_id)
            .ip(&ip_str)
            .save(&db.db)
            .await;
            
            Json(ApiResponse::success("Balance eliminado exitosamente".to_string()))
        }
        Err(e) => Json(ApiResponse::error(format!(
            "Error al eliminar el balance: {}",
            e
        ))),
    }
}
