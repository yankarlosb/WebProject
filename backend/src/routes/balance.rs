//! Rutas CRUD para Balances de Carga Docente y sus Fragmentos
//! 
//! Nueva arquitectura:
//! - Leader crea balance y selecciona asignaturas
//! - Se crean fragmentos automáticamente para cada asignatura
//! - SubjectLeaders llenan sus fragmentos correspondientes

use crate::utils::jwt::{AuthenticatedUser, LeaderUser, LeaderOrSubjectLeaderUser};
use crate::utils::audit;
use crate::database::audit_logs::{EventType, AuditCategory, EntityType};
use crate::*;
use crate::types::{ApiResponse, ApiResponseWithData};
use crate::database::{balances, balance_fragments, asignaturas, usuarios};
use rocket::{post, get, put, delete};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::net::SocketAddr;

// ============================================================================
// ESTRUCTURAS DE REQUEST/RESPONSE
// ============================================================================

/// Asignatura seleccionada para incluir en el balance
#[derive(Debug, Deserialize)]
pub struct SelectedSubject {
    pub asignatura_id: i32,
}

/// Período no académico (vacaciones, días feriados, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonAcademicPeriod {
    pub start: String,  // 'YYYY-MM-DD'
    pub end: String,    // 'YYYY-MM-DD'
    pub name: String,   // Descripción del período
}

/// Request para crear un balance (Leader)
#[derive(Debug, Deserialize)]
pub struct CreateBalanceRequest {
    pub academic_year: String,          // '1ro', '2do', '3ro', '4to'
    pub period: String,                 // '1ero', '2do'
    pub academic_year_text: String,     // '2025-2026'
    pub start_date: String,             // 'YYYY-MM-DD'
    pub weeks: i32,                     // Número de semanas
    pub deadline: Option<String>,       // Fecha límite opcional 'YYYY-MM-DD'
    pub allow_leader_edit: Option<bool>, // Si el leader puede editar fragmentos
    pub asignaturas: Vec<SelectedSubject>, // Asignaturas a incluir
    pub non_academic_periods: Option<Vec<NonAcademicPeriod>>, // Períodos no académicos
}

/// Request para actualizar metadatos del balance (Leader)
#[derive(Debug, Deserialize)]
pub struct UpdateBalanceRequest {
    pub academic_year: Option<String>,
    pub period: Option<String>,
    pub academic_year_text: Option<String>,
    pub start_date: Option<String>,
    pub weeks: Option<i32>,
    pub deadline: Option<String>,
    pub allow_leader_edit: Option<bool>,
    pub status: Option<String>,
    pub non_academic_periods: Option<Vec<NonAcademicPeriod>>, // Períodos no académicos
}

/// Request para actualizar un fragmento (SubjectLeader)
#[derive(Debug, Deserialize)]
pub struct UpdateFragmentRequest {
    pub data: serde_json::Value,  // Distribución semanal de la asignatura
    pub status: Option<String>,   // 'pending', 'in_progress', 'completed'
}

/// Información del SubjectLeader para respuesta
#[derive(Debug, Serialize)]
pub struct SubjectLeaderInfo {
    pub id: i32,
    pub user_name: String,
    pub name: String,
    pub email: String,
}

/// Información de la asignatura para respuesta
#[derive(Debug, Serialize)]
pub struct AsignaturaInfo {
    pub id: i32,
    pub name: String,
    pub year: String,
    pub semester: String,
    pub hours: i32,
    // Cantidades planificadas por tipo de clase
    pub c: Option<i32>,   // Conferencias
    pub cp: Option<i32>,  // Clases Prácticas
    pub s: Option<i32>,   // Seminarios
    pub pl: Option<i32>,  // Prácticas de Laboratorio
    pub te: Option<i32>,  // Trabajo Extraclase
    pub t: Option<i32>,   // Taller
    pub pp: Option<i32>,  // Pruebas Parciales
    pub ec: Option<i32>,  // Encuentro Comprobatorio
    pub tc: Option<i32>,  // Trabajo de Curso
    pub ef: Option<i32>,  // Examen Final
}

/// Respuesta de fragmento con información completa
#[derive(Debug, Serialize)]
pub struct FragmentResponse {
    pub id: i32,
    pub balance_id: i32,
    pub asignatura_id: i32,
    pub asignatura: Option<AsignaturaInfo>,
    pub subject_leader_id: Option<i32>,
    pub subject_leader: Option<SubjectLeaderInfo>,
    pub status: String,
    pub data: serde_json::Value,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Respuesta de balance con fragmentos
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
    pub status: String,
    pub deadline: Option<String>,
    pub allow_leader_edit: bool,
    pub non_academic_periods: Vec<NonAcademicPeriod>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub fragments: Vec<FragmentResponse>,
    pub progress: BalanceProgress,
}

/// Progreso del balance
#[derive(Debug, Serialize)]
pub struct BalanceProgress {
    pub total: usize,
    pub pending: usize,
    pub in_progress: usize,
    pub completed: usize,
    pub percentage: f32,
}

/// Respuesta simple de balance (sin fragmentos, para listados)
#[derive(Debug, Serialize)]
pub struct BalanceListItem {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub academic_year: String,
    pub period: String,
    pub academic_year_text: String,
    pub start_date: String,
    pub weeks: i32,
    pub status: String,
    pub deadline: Option<String>,
    pub non_academic_periods: Vec<NonAcademicPeriod>,
    pub created_at: Option<String>,
    pub progress: BalanceProgress,
}

/// Fragmento pendiente para dashboard de SubjectLeader
#[derive(Debug, Serialize)]
pub struct PendingFragment {
    pub fragment_id: i32,
    pub balance_id: i32,
    pub balance_name: String,
    pub asignatura_id: i32,
    pub asignatura_name: String,
    pub status: String,
    pub deadline: Option<String>,
}

// ============================================================================
// FUNCIONES AUXILIARES
// ============================================================================

/// Genera el nombre del balance automáticamente
fn generate_balance_name(academic_year: &str, period: &str, academic_year_text: &str) -> String {
    format!("{} Año - Período {} ({})", academic_year, period, academic_year_text)
}

/// Calcula el progreso de un balance basado en sus fragmentos
fn calculate_progress(fragments: &[balance_fragments::Model]) -> BalanceProgress {
    let total = fragments.len();
    let pending = fragments.iter().filter(|f| f.status == "pending").count();
    let in_progress = fragments.iter().filter(|f| f.status == "in_progress").count();
    let completed = fragments.iter().filter(|f| f.status == "completed").count();
    let percentage = if total > 0 {
        (completed as f32 / total as f32) * 100.0
    } else {
        0.0
    };

    BalanceProgress {
        total,
        pending,
        in_progress,
        completed,
        percentage,
    }
}

// ============================================================================
// RUTAS DE BALANCE (Leader)
// ============================================================================

/// Listar todos los balances
/// - Leader: ve todos los balances
/// - SubjectLeader: ve balances donde tiene fragmentos asignados
#[get("/balances")]
pub async fn list_balances(
    db: &State<AppState>,
    user: AuthenticatedUser,
) -> Json<ApiResponseWithData<Vec<BalanceListItem>>> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let user_role = &user.0.role;

    let balances_result = if user_role == "leader" || user_role == "admin" {
        // Leader/Admin ve todos los balances
        balances::Entity::find()
            .order_by_desc(balances::Column::CreatedAt)
            .all(&db.db)
            .await
    } else {
        // SubjectLeader ve solo balances donde tiene fragmentos
        balances::Entity::find()
            .inner_join(balance_fragments::Entity)
            .filter(balance_fragments::Column::SubjectLeaderId.eq(user_id))
            .order_by_desc(balances::Column::CreatedAt)
            .all(&db.db)
            .await
    };

    match balances_result {
        Ok(balances_list) => {
            let mut responses = Vec::new();
            
            for balance in balances_list {
                // Obtener fragmentos para calcular progreso
                let fragments = balance_fragments::Entity::find()
                    .filter(balance_fragments::Column::BalanceId.eq(balance.id))
                    .all(&db.db)
                    .await
                    .unwrap_or_default();

                let progress = calculate_progress(&fragments);

                // Parsear períodos no académicos del JSON
                let non_academic_periods: Vec<NonAcademicPeriod> = 
                    serde_json::from_value(balance.non_academic_periods.clone())
                        .unwrap_or_default();

                responses.push(BalanceListItem {
                    id: balance.id,
                    user_id: balance.user_id,
                    name: balance.name,
                    academic_year: balance.academic_year,
                    period: balance.period,
                    academic_year_text: balance.academic_year_text,
                    start_date: balance.start_date.to_string(),
                    weeks: balance.weeks,
                    status: balance.status,
                    deadline: balance.deadline.map(|d| d.to_string()),
                    non_academic_periods,
                    created_at: balance.created_at.map(|dt| dt.to_string()),
                    progress,
                });
            }

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

/// Obtener un balance con todos sus fragmentos
#[get("/balances/<balance_id>")]
pub async fn get_balance(
    balance_id: i32,
    db: &State<AppState>,
    user: AuthenticatedUser,
) -> Json<ApiResponseWithData<BalanceResponse>> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let user_role = &user.0.role;

    // Obtener el balance
    let balance_result = balances::Entity::find_by_id(balance_id)
        .one(&db.db)
        .await;

    let balance = match balance_result {
        Ok(Some(b)) => b,
        Ok(None) => return Json(ApiResponseWithData::error("Balance no encontrado".to_string())),
        Err(e) => return Json(ApiResponseWithData::error(format!("Error: {}", e))),
    };

    // Verificar permisos: Leader ve todo, SubjectLeader solo si tiene fragmento
    if user_role != "leader" && user_role != "admin" {
        let has_fragment = balance_fragments::Entity::find()
            .filter(balance_fragments::Column::BalanceId.eq(balance_id))
            .filter(balance_fragments::Column::SubjectLeaderId.eq(user_id))
            .one(&db.db)
            .await
            .unwrap_or(None);

        if has_fragment.is_none() {
            return Json(ApiResponseWithData::error(
                "No tienes permiso para ver este balance".to_string(),
            ));
        }
    }

    // Obtener fragmentos con información de asignatura y subject_leader
    let fragments_result = balance_fragments::Entity::find()
        .filter(balance_fragments::Column::BalanceId.eq(balance_id))
        .all(&db.db)
        .await;

    let fragments = match fragments_result {
        Ok(f) => f,
        Err(e) => return Json(ApiResponseWithData::error(format!("Error al obtener fragmentos: {}", e))),
    };

    let mut fragment_responses = Vec::new();

    for fragment in &fragments {
        // Obtener info de asignatura
        let asignatura_info = asignaturas::Entity::find_by_id(fragment.asignatura_id)
            .one(&db.db)
            .await
            .ok()
            .flatten()
            .map(|a| AsignaturaInfo {
                id: a.id,
                name: a.name,
                year: a.year,
                semester: a.semester,
                hours: a.hours,
                c: a.c,
                cp: a.cp,
                s: a.s,
                pl: a.pl,
                te: a.te,
                t: a.t,
                pp: a.pp,
                ec: a.ec,
                tc: a.tc,
                ef: a.ef,
            });

        // Obtener info de subject_leader
        let subject_leader_info = if let Some(leader_id) = fragment.subject_leader_id {
            usuarios::Entity::find_by_id(leader_id)
                .one(&db.db)
                .await
                .ok()
                .flatten()
                .map(|u| SubjectLeaderInfo {
                    id: u.id,
                    user_name: u.user_name,
                    name: u.name,
                    email: u.email,
                })
        } else {
            None
        };

        fragment_responses.push(FragmentResponse {
            id: fragment.id,
            balance_id: fragment.balance_id,
            asignatura_id: fragment.asignatura_id,
            asignatura: asignatura_info,
            subject_leader_id: fragment.subject_leader_id,
            subject_leader: subject_leader_info,
            status: fragment.status.clone(),
            data: fragment.data.clone(),
            completed_at: fragment.completed_at.map(|dt| dt.to_string()),
            created_at: fragment.created_at.map(|dt| dt.to_string()),
            updated_at: fragment.updated_at.map(|dt| dt.to_string()),
        });
    }

    let progress = calculate_progress(&fragments);

    // Parsear períodos no académicos del JSON
    let non_academic_periods: Vec<NonAcademicPeriod> = 
        serde_json::from_value(balance.non_academic_periods.clone())
            .unwrap_or_default();

    Json(ApiResponseWithData::success(
        "Balance obtenido exitosamente".to_string(),
        BalanceResponse {
            id: balance.id,
            user_id: balance.user_id,
            name: balance.name,
            academic_year: balance.academic_year,
            period: balance.period,
            academic_year_text: balance.academic_year_text,
            start_date: balance.start_date.to_string(),
            weeks: balance.weeks,
            status: balance.status,
            deadline: balance.deadline.map(|d| d.to_string()),
            allow_leader_edit: balance.allow_leader_edit,
            non_academic_periods,
            created_at: balance.created_at.map(|dt| dt.to_string()),
            updated_at: balance.updated_at.map(|dt| dt.to_string()),
            fragments: fragment_responses,
            progress,
        },
    ))
}

/// Crear un nuevo balance con sus fragmentos (Solo Leader)
#[post("/balances", format = "json", data = "<balance_data>")]
pub async fn create_balance(
    balance_data: Json<CreateBalanceRequest>,
    db: &State<AppState>,
    user: LeaderUser,
    remote_addr: Option<SocketAddr>,
) -> Json<ApiResponseWithData<BalanceResponse>> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let data = balance_data.into_inner();
    let ip_str = remote_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());

    // Validar que hay al menos una asignatura
    if data.asignaturas.is_empty() {
        return Json(ApiResponseWithData::error(
            "Debe seleccionar al menos una asignatura".to_string(),
        ));
    }

    // Parsear fecha de inicio
    let start_date = match NaiveDate::parse_from_str(&data.start_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return Json(ApiResponseWithData::error(
            "Formato de fecha de inicio inválido. Use YYYY-MM-DD".to_string(),
        )),
    };

    // Parsear deadline si existe
    let deadline = if let Some(ref dl) = data.deadline {
        match NaiveDate::parse_from_str(dl, "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(_) => return Json(ApiResponseWithData::error(
                "Formato de fecha límite inválido. Use YYYY-MM-DD".to_string(),
            )),
        }
    } else {
        None
    };

    // Generar nombre
    let name = generate_balance_name(&data.academic_year, &data.period, &data.academic_year_text);

    // Crear el balance
    let new_balance = balances::ActiveModel {
        user_id: Set(user_id),
        name: Set(name.clone()),
        academic_year: Set(data.academic_year.clone()),
        period: Set(data.period.clone()),
        academic_year_text: Set(data.academic_year_text.clone()),
        start_date: Set(start_date),
        weeks: Set(data.weeks),
        status: Set("draft".to_string()),
        deadline: Set(deadline),
        allow_leader_edit: Set(data.allow_leader_edit.unwrap_or(false)),
        subjects: Set(serde_json::json!([])), // Deprecated, usamos fragments
        non_academic_periods: Set(serde_json::to_value(&data.non_academic_periods.clone().unwrap_or_default()).unwrap_or(serde_json::json!([]))),
        ..Default::default()
    };

    let inserted_balance = match new_balance.insert(&db.db).await {
        Ok(b) => b,
        Err(e) => return Json(ApiResponseWithData::error(format!(
            "Error al crear el balance: {}", e
        ))),
    };

    // Crear fragmentos para cada asignatura
    let mut fragment_responses = Vec::new();
    let mut errors = Vec::new();

    for selected in data.asignaturas {
        // Obtener la asignatura y su leader_id
        let asignatura = match asignaturas::Entity::find_by_id(selected.asignatura_id)
            .one(&db.db)
            .await
        {
            Ok(Some(a)) => a,
            Ok(None) => {
                errors.push(format!("Asignatura {} no encontrada", selected.asignatura_id));
                continue;
            }
            Err(e) => {
                errors.push(format!("Error al buscar asignatura: {}", e));
                continue;
            }
        };

        // El subject_leader es el leader_id de la asignatura
        let subject_leader_id = if asignatura.leader_id > 0 {
            Some(asignatura.leader_id)
        } else {
            None
        };

        // Crear el fragmento
        let new_fragment = balance_fragments::ActiveModel {
            balance_id: Set(inserted_balance.id),
            asignatura_id: Set(asignatura.id),
            subject_leader_id: Set(subject_leader_id),
            status: Set("pending".to_string()),
            data: Set(serde_json::json!({})),
            ..Default::default()
        };

        match new_fragment.insert(&db.db).await {
            Ok(fragment) => {
                // Obtener info del subject_leader
                let subject_leader_info = if let Some(leader_id) = subject_leader_id {
                    usuarios::Entity::find_by_id(leader_id)
                        .one(&db.db)
                        .await
                        .ok()
                        .flatten()
                        .map(|u| SubjectLeaderInfo {
                            id: u.id,
                            user_name: u.user_name,
                            name: u.name,
                            email: u.email,
                        })
                } else {
                    None
                };

                fragment_responses.push(FragmentResponse {
                    id: fragment.id,
                    balance_id: fragment.balance_id,
                    asignatura_id: fragment.asignatura_id,
                    asignatura: Some(AsignaturaInfo {
                        id: asignatura.id,
                        name: asignatura.name.clone(),
                        year: asignatura.year.clone(),
                        semester: asignatura.semester.clone(),
                        hours: asignatura.hours,
                        c: asignatura.c,
                        cp: asignatura.cp,
                        s: asignatura.s,
                        pl: asignatura.pl,
                        te: asignatura.te,
                        t: asignatura.t,
                        pp: asignatura.pp,
                        ec: asignatura.ec,
                        tc: asignatura.tc,
                        ef: asignatura.ef,
                    }),
                    subject_leader_id: fragment.subject_leader_id,
                    subject_leader: subject_leader_info,
                    status: fragment.status,
                    data: fragment.data,
                    completed_at: fragment.completed_at.map(|dt| dt.to_string()),
                    created_at: fragment.created_at.map(|dt| dt.to_string()),
                    updated_at: fragment.updated_at.map(|dt| dt.to_string()),
                });
            }
            Err(e) => {
                errors.push(format!("Error al crear fragmento para asignatura {}: {}", asignatura.id, e));
            }
        }
    }

    // Registrar en auditoría
    let _ = audit::AuditLogBuilder::new(
        EventType::Create,
        AuditCategory::Functional,
        format!("Leader '{}' creó el balance '{}' con {} fragmentos", user.0.user_name, name, fragment_responses.len()),
    )
    .user(user_id, &user.0.user_name)
    .entity(EntityType::Balance, inserted_balance.id)
    .ip(&ip_str)
    .save(&db.db)
    .await;

    let progress = BalanceProgress {
        total: fragment_responses.len(),
        pending: fragment_responses.len(),
        in_progress: 0,
        completed: 0,
        percentage: 0.0,
    };

    // Obtener períodos no académicos del balance creado
    let non_academic_periods: Vec<NonAcademicPeriod> = 
        serde_json::from_value(inserted_balance.non_academic_periods.clone())
            .unwrap_or_default();

    let message = if errors.is_empty() {
        "Balance creado exitosamente".to_string()
    } else {
        format!("Balance creado con advertencias: {}", errors.join(", "))
    };

    Json(ApiResponseWithData::success(
        message,
        BalanceResponse {
            id: inserted_balance.id,
            user_id: inserted_balance.user_id,
            name: inserted_balance.name,
            academic_year: inserted_balance.academic_year,
            period: inserted_balance.period,
            academic_year_text: inserted_balance.academic_year_text,
            start_date: inserted_balance.start_date.to_string(),
            weeks: inserted_balance.weeks,
            status: inserted_balance.status,
            deadline: inserted_balance.deadline.map(|d| d.to_string()),
            allow_leader_edit: inserted_balance.allow_leader_edit,
            non_academic_periods,
            created_at: inserted_balance.created_at.map(|dt| dt.to_string()),
            updated_at: inserted_balance.updated_at.map(|dt| dt.to_string()),
            fragments: fragment_responses,
            progress,
        },
    ))
}

/// Actualizar metadatos de un balance (Solo Leader)
#[put("/balances/<balance_id>", format = "json", data = "<balance_data>")]
pub async fn update_balance(
    balance_id: i32,
    balance_data: Json<UpdateBalanceRequest>,
    db: &State<AppState>,
    user: LeaderUser,
    remote_addr: Option<SocketAddr>,
) -> Json<ApiResponse> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let data = balance_data.into_inner();
    let ip_str = remote_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());

    // Obtener el balance
    let balance = match balances::Entity::find_by_id(balance_id)
        .one(&db.db)
        .await
    {
        Ok(Some(b)) => b,
        Ok(None) => return Json(ApiResponse::error("Balance no encontrado".to_string())),
        Err(e) => return Json(ApiResponse::error(format!("Error: {}", e))),
    };

    let mut active_model: balances::ActiveModel = balance.into();

    // Actualizar campos si se proporcionan
    if let Some(ay) = data.academic_year {
        active_model.academic_year = Set(ay);
    }
    if let Some(p) = data.period {
        active_model.period = Set(p);
    }
    if let Some(ayt) = data.academic_year_text {
        active_model.academic_year_text = Set(ayt);
    }
    if let Some(sd) = data.start_date {
        match NaiveDate::parse_from_str(&sd, "%Y-%m-%d") {
            Ok(date) => active_model.start_date = Set(date),
            Err(_) => return Json(ApiResponse::error("Formato de fecha inválido".to_string())),
        }
    }
    if let Some(w) = data.weeks {
        active_model.weeks = Set(w);
    }
    if let Some(dl) = data.deadline {
        match NaiveDate::parse_from_str(&dl, "%Y-%m-%d") {
            Ok(date) => active_model.deadline = Set(Some(date)),
            Err(_) => return Json(ApiResponse::error("Formato de fecha límite inválido".to_string())),
        }
    }
    if let Some(ale) = data.allow_leader_edit {
        active_model.allow_leader_edit = Set(ale);
    }
    if let Some(s) = data.status {
        if !["draft", "in_progress", "completed"].contains(&s.as_str()) {
            return Json(ApiResponse::error("Estado inválido".to_string()));
        }
        active_model.status = Set(s);
    }
    if let Some(nap) = data.non_academic_periods {
        active_model.non_academic_periods = Set(
            serde_json::to_value(&nap).unwrap_or(serde_json::json!([]))
        );
    }

    active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

    match active_model.update(&db.db).await {
        Ok(_) => {
            let _ = audit::AuditLogBuilder::new(
                EventType::Update,
                AuditCategory::Functional,
                format!("Leader '{}' actualizó el balance ID {}", user.0.user_name, balance_id),
            )
            .user(user_id, &user.0.user_name)
            .entity(EntityType::Balance, balance_id)
            .ip(&ip_str)
            .save(&db.db)
            .await;

            Json(ApiResponse::success("Balance actualizado exitosamente".to_string()))
        }
        Err(e) => Json(ApiResponse::error(format!("Error al actualizar: {}", e))),
    }
}

/// Eliminar un balance y sus fragmentos (Solo Leader)
#[delete("/balances/<balance_id>")]
pub async fn delete_balance(
    balance_id: i32,
    db: &State<AppState>,
    user: LeaderUser,
    remote_addr: Option<SocketAddr>,
) -> Json<ApiResponse> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let ip_str = remote_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());

    let balance = match balances::Entity::find_by_id(balance_id)
        .one(&db.db)
        .await
    {
        Ok(Some(b)) => b,
        Ok(None) => return Json(ApiResponse::error("Balance no encontrado".to_string())),
        Err(e) => return Json(ApiResponse::error(format!("Error: {}", e))),
    };

    let balance_name = balance.name.clone();

    // Los fragmentos se eliminan automáticamente por ON DELETE CASCADE
    match balance.delete(&db.db).await {
        Ok(_) => {
            let _ = audit::AuditLogBuilder::new(
                EventType::Delete,
                AuditCategory::Functional,
                format!("Leader '{}' eliminó el balance '{}' (ID: {})", user.0.user_name, balance_name, balance_id),
            )
            .user(user_id, &user.0.user_name)
            .entity(EntityType::Balance, balance_id)
            .ip(&ip_str)
            .save(&db.db)
            .await;

            Json(ApiResponse::success("Balance eliminado exitosamente".to_string()))
        }
        Err(e) => Json(ApiResponse::error(format!("Error al eliminar: {}", e))),
    }
}

// ============================================================================
// RUTAS DE FRAGMENTOS (SubjectLeader)
// ============================================================================

/// Obtener fragmentos pendientes del usuario actual (para Dashboard)
#[get("/fragments/pending")]
pub async fn get_pending_fragments(
    db: &State<AppState>,
    user: AuthenticatedUser,
) -> Json<ApiResponseWithData<Vec<PendingFragment>>> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);

    let fragments = match balance_fragments::Entity::find()
        .filter(balance_fragments::Column::SubjectLeaderId.eq(user_id))
        .filter(balance_fragments::Column::Status.ne("completed"))
        .all(&db.db)
        .await
    {
        Ok(f) => f,
        Err(e) => return Json(ApiResponseWithData::error(format!("Error: {}", e))),
    };

    let mut pending = Vec::new();

    for fragment in fragments {
        // Obtener balance
        let balance = balances::Entity::find_by_id(fragment.balance_id)
            .one(&db.db)
            .await
            .ok()
            .flatten();

        // Obtener asignatura
        let asignatura = asignaturas::Entity::find_by_id(fragment.asignatura_id)
            .one(&db.db)
            .await
            .ok()
            .flatten();

        if let (Some(b), Some(a)) = (balance, asignatura) {
            pending.push(PendingFragment {
                fragment_id: fragment.id,
                balance_id: fragment.balance_id,
                balance_name: b.name,
                asignatura_id: fragment.asignatura_id,
                asignatura_name: a.name,
                status: fragment.status,
                deadline: b.deadline.map(|d| d.to_string()),
            });
        }
    }

    Json(ApiResponseWithData::success(
        "Fragmentos pendientes obtenidos".to_string(),
        pending,
    ))
}

/// Obtener un fragmento específico
#[get("/balances/<balance_id>/fragments/<asignatura_id>")]
pub async fn get_fragment(
    balance_id: i32,
    asignatura_id: i32,
    db: &State<AppState>,
    user: LeaderOrSubjectLeaderUser,
) -> Result<Json<ApiResponseWithData<FragmentResponse>>, (Status, Json<ApiResponseWithData<FragmentResponse>>)> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let user_role = &user.0.role;

    // Buscar el fragmento
    let fragment = match balance_fragments::Entity::find()
        .filter(balance_fragments::Column::BalanceId.eq(balance_id))
        .filter(balance_fragments::Column::AsignaturaId.eq(asignatura_id))
        .one(&db.db)
        .await
    {
        Ok(Some(f)) => f,
        Ok(None) => return Err((Status::NotFound, Json(ApiResponseWithData::error("Fragmento no encontrado".to_string())))),
        Err(e) => return Err((Status::InternalServerError, Json(ApiResponseWithData::error(format!("Error: {}", e))))),
    };

    // Verificar permisos: Leader ve todo, SubjectLeader solo su fragmento
    if user_role != "leader" && user_role != "admin" {
        if fragment.subject_leader_id != Some(user_id) {
            return Err((Status::Forbidden, Json(ApiResponseWithData::error(
                "No tienes permiso para ver este fragmento".to_string(),
            ))));
        }
    }

    // Obtener info de asignatura
    let asignatura_info = asignaturas::Entity::find_by_id(fragment.asignatura_id)
        .one(&db.db)
        .await
        .ok()
        .flatten()
        .map(|a| AsignaturaInfo {
            id: a.id,
            name: a.name,
            year: a.year,
            semester: a.semester,
            hours: a.hours,
            c: a.c,
            cp: a.cp,
            s: a.s,
            pl: a.pl,
            te: a.te,
            t: a.t,
            pp: a.pp,
            ec: a.ec,
            tc: a.tc,
            ef: a.ef,
        });

    // Obtener info del subject_leader
    let subject_leader_info = if let Some(leader_id) = fragment.subject_leader_id {
        usuarios::Entity::find_by_id(leader_id)
            .one(&db.db)
            .await
            .ok()
            .flatten()
            .map(|u| SubjectLeaderInfo {
                id: u.id,
                user_name: u.user_name,
                name: u.name,
                email: u.email,
            })
    } else {
        None
    };

    Ok(Json(ApiResponseWithData::success(
        "Fragmento obtenido exitosamente".to_string(),
        FragmentResponse {
            id: fragment.id,
            balance_id: fragment.balance_id,
            asignatura_id: fragment.asignatura_id,
            asignatura: asignatura_info,
            subject_leader_id: fragment.subject_leader_id,
            subject_leader: subject_leader_info,
            status: fragment.status,
            data: fragment.data,
            completed_at: fragment.completed_at.map(|dt| dt.to_string()),
            created_at: fragment.created_at.map(|dt| dt.to_string()),
            updated_at: fragment.updated_at.map(|dt| dt.to_string()),
        },
    )))
}

/// Actualizar un fragmento (SubjectLeader edita su parte)
#[put("/balances/<balance_id>/fragments/<asignatura_id>", format = "json", data = "<fragment_data>")]
pub async fn update_fragment(
    balance_id: i32,
    asignatura_id: i32,
    fragment_data: Json<UpdateFragmentRequest>,
    db: &State<AppState>,
    user: LeaderOrSubjectLeaderUser,
    remote_addr: Option<SocketAddr>,
) -> Result<Json<ApiResponse>, (Status, Json<ApiResponse>)> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let user_role = &user.0.role;
    let data = fragment_data.into_inner();
    let ip_str = remote_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());

    // Buscar el fragmento
    let fragment = match balance_fragments::Entity::find()
        .filter(balance_fragments::Column::BalanceId.eq(balance_id))
        .filter(balance_fragments::Column::AsignaturaId.eq(asignatura_id))
        .one(&db.db)
        .await
    {
        Ok(Some(f)) => f,
        Ok(None) => return Err((Status::NotFound, Json(ApiResponse::error("Fragmento no encontrado".to_string())))),
        Err(e) => return Err((Status::InternalServerError, Json(ApiResponse::error(format!("Error: {}", e))))),
    };

    // Verificar permisos
    let is_leader = user_role == "leader" || user_role == "admin";
    let is_owner = fragment.subject_leader_id == Some(user_id);

    // Leader y Admin siempre pueden editar cualquier fragmento
    // SubjectLeader solo puede editar su propio fragmento
    if !is_leader && !is_owner {
        return Err((Status::Forbidden, Json(ApiResponse::error(
            "No tienes permiso para editar este fragmento".to_string(),
        ))));
    }

    // Actualizar el fragmento
    let mut active_model: balance_fragments::ActiveModel = fragment.into();
    active_model.data = Set(data.data);
    active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

    if let Some(status) = data.status {
        if !["pending", "in_progress", "completed"].contains(&status.as_str()) {
            return Err((Status::BadRequest, Json(ApiResponse::error("Estado inválido".to_string()))));
        }
        active_model.status = Set(status.clone());

        // Si se marca como completado, guardar timestamp
        if status == "completed" {
            active_model.completed_at = Set(Some(chrono::Utc::now().naive_utc()));
        }
    }

    match active_model.update(&db.db).await {
        Ok(_) => {
            // Obtener nombre de asignatura para el log
            let asignatura_name = asignaturas::Entity::find_by_id(asignatura_id)
                .one(&db.db)
                .await
                .ok()
                .flatten()
                .map(|a| a.name)
                .unwrap_or_else(|| "Desconocida".to_string());

            let _ = audit::AuditLogBuilder::new(
                EventType::Update,
                AuditCategory::Functional,
                format!("Usuario '{}' actualizó fragmento de '{}' en balance ID {}", 
                    user.0.user_name, asignatura_name, balance_id),
            )
            .user(user_id, &user.0.user_name)
            .entity(EntityType::Balance, balance_id)
            .ip(&ip_str)
            .save(&db.db)
            .await;

            // Actualizar estado del balance si todos los fragmentos están completos
            update_balance_status_if_complete(&db.db, balance_id).await;

            Ok(Json(ApiResponse::success("Fragmento actualizado exitosamente".to_string())))
        }
        Err(e) => Err((Status::InternalServerError, Json(ApiResponse::error(format!("Error al actualizar: {}", e))))),
    }
}

/// Actualiza el estado del balance a 'completed' si todos los fragmentos están completos
async fn update_balance_status_if_complete(db: &DatabaseConnection, balance_id: i32) {
    let fragments = balance_fragments::Entity::find()
        .filter(balance_fragments::Column::BalanceId.eq(balance_id))
        .all(db)
        .await
        .unwrap_or_default();

    let all_completed = !fragments.is_empty() && 
        fragments.iter().all(|f| f.status == "completed");

    if all_completed {
        if let Ok(Some(balance)) = balances::Entity::find_by_id(balance_id).one(db).await {
            let mut active_model: balances::ActiveModel = balance.into();
            active_model.status = Set("completed".to_string());
            active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
            let _ = active_model.update(db).await;
        }
    } else {
        // Si hay al menos un fragmento in_progress, actualizar balance a in_progress
        let any_in_progress = fragments.iter().any(|f| f.status == "in_progress" || f.status == "completed");
        if any_in_progress {
            if let Ok(Some(balance)) = balances::Entity::find_by_id(balance_id).one(db).await {
                if balance.status == "draft" {
                    let mut active_model: balances::ActiveModel = balance.into();
                    active_model.status = Set("in_progress".to_string());
                    active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
                    let _ = active_model.update(db).await;
                }
            }
        }
    }
}

// ============================================================================
// EXPORTACIÓN A EXCEL
// ============================================================================

use crate::utils::excel_export::{
    BalanceExportConfig, FragmentExportData, ActivityPlan, 
    generate_balance_excel, parse_fragment_data, parse_consultas_data, parse_examenes_data
};
use rocket::http::ContentType;

/// Custom responder for Excel file download
pub struct ExcelFile {
    pub data: Vec<u8>,
    pub filename: String,
}

impl<'r> rocket::response::Responder<'r, 'static> for ExcelFile {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            .header(ContentType::new("application", "vnd.openxmlformats-officedocument.spreadsheetml.sheet"))
            .raw_header("Content-Disposition", format!("attachment; filename=\"{}\"", self.filename))
            .sized_body(self.data.len(), std::io::Cursor::new(self.data))
            .ok()
    }
}

/// Exportar balance a Excel
/// GET /api/balances/<id>/export
#[get("/balances/<balance_id>/export")]
pub async fn export_balance_excel(
    db: &State<AppState>,
    user: LeaderOrSubjectLeaderUser,
    balance_id: i32,
) -> Result<ExcelFile, (Status, Json<ApiResponse>)> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let user_role = &user.0.role;

    // Buscar el balance
    let balance = balances::Entity::find_by_id(balance_id)
        .one(&db.db)
        .await
        .map_err(|e| (Status::InternalServerError, Json(ApiResponse::error(format!("Error de base de datos: {}", e)))))?
        .ok_or_else(|| (Status::NotFound, Json(ApiResponse::error("Balance no encontrado".to_string()))))?;

    // Verificar permisos
    // SubjectLeader solo puede exportar si tiene fragmentos asignados
    if user_role != "leader" && user_role != "admin" {
        let has_fragment = balance_fragments::Entity::find()
            .filter(balance_fragments::Column::BalanceId.eq(balance_id))
            .filter(balance_fragments::Column::SubjectLeaderId.eq(user_id))
            .one(&db.db)
            .await
            .map_err(|e| (Status::InternalServerError, Json(ApiResponse::error(format!("Error: {}", e)))))?;
        
        if has_fragment.is_none() {
            return Err((Status::Forbidden, Json(ApiResponse::error("No tiene permisos para exportar este balance".to_string()))));
        }
    }

    // Obtener fragmentos con datos de asignatura
    let fragments = balance_fragments::Entity::find()
        .filter(balance_fragments::Column::BalanceId.eq(balance_id))
        .all(&db.db)
        .await
        .map_err(|e| (Status::InternalServerError, Json(ApiResponse::error(format!("Error obteniendo fragmentos: {}", e)))))?;

    // Construir datos de exportación
    let mut fragment_data = Vec::new();
    for fragment in &fragments {
        // Obtener info de la asignatura
        let asignatura = asignaturas::Entity::find_by_id(fragment.asignatura_id)
            .one(&db.db)
            .await
            .map_err(|e| (Status::InternalServerError, Json(ApiResponse::error(format!("Error: {}", e)))))?;

        if let Some(asig) = asignatura {
            let weekly_data = parse_fragment_data(&fragment.data, balance.weeks);
            let consultas_data = parse_consultas_data(&fragment.data, balance.weeks);
            let examenes_data = parse_examenes_data(&fragment.data, balance.weeks);
            
            fragment_data.push(FragmentExportData {
                name: asig.name,
                hours: asig.hours,
                weekly_data,
                consultas_data,
                examenes_data,
                plan: ActivityPlan {
                    c: asig.c.unwrap_or(0),
                    cp: asig.cp.unwrap_or(0),
                    s: asig.s.unwrap_or(0),
                    pl: asig.pl.unwrap_or(0),
                    te: asig.te.unwrap_or(0),
                    t: asig.t.unwrap_or(0),
                    pp: asig.pp.unwrap_or(0),
                    ec: asig.ec.unwrap_or(0),
                    tc: asig.tc.unwrap_or(0),
                    ef: asig.ef.unwrap_or(0),
                },
            });
        }
    }

    // start_date is already NaiveDate (Date type from SeaORM)
    let start_date = balance.start_date;

    // Parse non_academic_periods from Json
    let non_academic_periods: Vec<(NaiveDate, NaiveDate, String)> = serde_json::from_value::<Vec<NonAcademicPeriod>>(balance.non_academic_periods.clone())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|p| {
            let start = NaiveDate::parse_from_str(&p.start, "%Y-%m-%d").ok()?;
            let end = NaiveDate::parse_from_str(&p.end, "%Y-%m-%d").ok()?;
            Some((start, end, p.name))
        })
        .collect();

    // Build export config
    let config = BalanceExportConfig {
        academic_year: balance.academic_year.clone(),
        period: balance.period.clone(),
        academic_year_text: balance.academic_year_text.clone(),
        start_date,
        weeks: balance.weeks,
        fragments: fragment_data,
        non_academic_periods,
    };

    // Generate Excel
    let excel_bytes = generate_balance_excel(&config)
        .map_err(|e| (Status::InternalServerError, Json(ApiResponse::error(format!("Error generando Excel: {}", e)))))?;

    // Generate filename
    let filename = format!(
        "Balance_de_carga_Diurno_{}_{}_{}.xlsx",
        balance.academic_year_text.replace("-", "_"),
        balance.period.replace(" ", "_"),
        balance.academic_year.replace(" ", "_")
    );

    Ok(ExcelFile { data: excel_bytes, filename })
}
