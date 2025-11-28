use crate::utils::jwt::{AdminUser, AuthenticatedUser, LeaderUser, SubjectLeaderUser, LeaderOrSubjectLeaderUser};
use crate::utils::validation::{validate_new_user, validate_profile, validate_subject, is_valid_password};
use crate::*;
use crate::types::{ApiResponse, ApiResponseWithData};
use crate::{usuarios, asignaturas};
use rocket::{post, get, put, delete};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    pub user_name: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[post("/create_user", format = "json", data = "<new_user>")]
pub async fn create_user(
    new_user: Json<NewUser>,
    db: &State<AppState>,
    _admin: AdminUser,
) -> Json<ApiResponse> {
    let user_name = new_user.user_name.trim();
    let name = new_user.name.trim();
    let email = new_user.email.trim();
    let password = &new_user.password;
    let role = &new_user.role;

    // Validar datos de entrada
    let validation = validate_new_user(user_name, name, email, password);
    if !validation.valid {
        return Json(ApiResponse::error(validation.error.unwrap_or_else(|| "Datos inválidos".to_string())));
    }

    match utils::db::create_user(&db.db, user_name, name, email, password, role).await {
        Ok(_) => Json(ApiResponse::success("Usuario creado exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al crear el usuario: {}", e))),
    }
}

#[post("/delete_user/<user_id>")]
pub async fn delete_user(
    user_id: i32,
    db: &State<AppState>,
    _admin: AdminUser,
) -> Json<ApiResponse> {
    match utils::db::delete_user(&db.db, user_id).await {
        Ok(_) => Json(ApiResponse::success("Usuario eliminado exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al eliminar el usuario: {}", e))),
    }
}

#[get("/list_users")]
pub async fn list_users(
    db: &State<AppState>,
    _admin: AdminUser,
) -> Json<ApiResponseWithData<Vec<usuarios::Model>>> {
    match utils::db::list_users(&db.db).await {
        Ok(users) => Json(ApiResponseWithData::success("Usuarios obtenidos exitosamente".to_string(), users)),
        Err(e) => Json(ApiResponseWithData::error(format!("Error al obtener los usuarios: {}", e))),
    }
}

#[post("/modify_user/<user_id>", format = "json", data = "<user_data>")]
pub async fn modify_user(
    user_id: i32,
    user_data: Json<usuarios::Model>,
    db: &State<AppState>,
    _admin: AdminUser,
) -> Json<ApiResponse> {
    match utils::db::modify_user(&db.db, user_id, &user_data.into_inner()).await {
        Ok(_) => Json(ApiResponse::success("Usuario modificado exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al modificar el usuario: {}", e))),
    }
}

// Endpoints para perfil de usuario autenticado
#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub name: String,
    pub email: String,
}

#[post("/update_profile", format = "json", data = "<profile_data>")]
pub async fn update_profile(
    profile_data: Json<UpdateProfileRequest>,
    db: &State<AppState>,
    user: AuthenticatedUser,
) -> Json<ApiResponse> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let name = profile_data.name.trim();
    let email = profile_data.email.trim();
    
    // Validar datos
    let validation = validate_profile(name, email);
    if !validation.valid {
        return Json(ApiResponse::error(validation.error.unwrap_or_else(|| "Datos inválidos".to_string())));
    }
    
    match utils::db::update_profile(&db.db, user_id, name, email).await {
        Ok(_) => Json(ApiResponse::success("Perfil actualizado exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al actualizar el perfil: {}", e))),
    }
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub new_password: String,
}

#[post("/change_password", format = "json", data = "<password_data>")]
pub async fn change_password(
    password_data: Json<ChangePasswordRequest>,
    db: &State<AppState>,
    user: AuthenticatedUser,
) -> Json<ApiResponse> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    
    // Validar contraseña
    if !is_valid_password(&password_data.new_password) {
        return Json(ApiResponse::error("Contraseña inválida (mínimo 8 caracteres)".to_string()));
    }
    
    match utils::db::change_user_password(&db.db, user_id, &password_data.new_password).await {
        Ok(_) => Json(ApiResponse::success("Contraseña cambiada exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al cambiar la contraseña: {}", e))),
    }
}

// ============================================================================
// ENDPOINTS DE ASIGNATURAS
// ============================================================================

#[derive(Deserialize)]
pub struct CreateAsignaturaRequest {
    pub leader_user_name: String,  // Username del jefe de asignatura
    pub name: String,
    pub year: String,
    pub semester: String,
}

/// Crear asignatura - Solo Leaders
/// Los datos de horas y actividades se inicializan en valores por defecto
#[post("/asignaturas/create", format = "json", data = "<asignatura_data>")]
pub async fn create_asignatura(
    asignatura_data: Json<CreateAsignaturaRequest>,
    db: &State<AppState>,
    _leader: LeaderUser,
) -> Json<ApiResponse> {
    // Validar nombre de asignatura
    let validation = validate_subject(&asignatura_data.name);
    if !validation.valid {
        return Json(ApiResponse::error(validation.error.unwrap_or_else(|| "Nombre de asignatura inválido".to_string())));
    }
    
    match utils::db::create_asignatura(&db.db, &asignatura_data.into_inner()).await {
        Ok(_) => Json(ApiResponse::success("Asignatura creada exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al crear la asignatura: {}", e))),
    }
}

/// Listar asignaturas
/// - Leader: ve todas las asignaturas
/// - SubjectLeader: solo ve sus asignaturas (donde leader_id = user_id)
#[get("/asignaturas/list")]
pub async fn list_asignaturas(
    db: &State<AppState>,
    user: LeaderOrSubjectLeaderUser,
) -> Json<ApiResponseWithData<Vec<asignaturas::Model>>> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    let role = &user.0.role;

    match utils::db::list_asignaturas(&db.db, user_id, role).await {
        Ok(asignaturas) => Json(ApiResponseWithData::success("Asignaturas obtenidas exitosamente".to_string(), asignaturas)),
        Err(e) => Json(ApiResponseWithData::error(format!("Error al obtener las asignaturas: {}", e))),
    }
}

#[derive(Deserialize)]
pub struct UpdateAsignaturaRequest {
    pub name: String,
    pub year: String,
    pub semester: String,
    pub c: Option<i32>,
    pub cp: Option<i32>,
    pub s: Option<i32>,
    pub pl: Option<i32>,
    pub te: Option<i32>,
    pub t: Option<i32>,
    pub pp: Option<i32>,
    pub ec: Option<i32>,
    pub tc: Option<i32>,
    pub ef: Option<i32>,
    pub hours: i32,
    pub weeks: Option<i32>,
}

/// Actualizar asignatura - Solo SubjectLeaders (para sus asignaturas)
#[put("/asignaturas/update/<asignatura_id>", format = "json", data = "<asignatura_data>")]
pub async fn update_asignatura(
    asignatura_id: i32,
    asignatura_data: Json<UpdateAsignaturaRequest>,
    db: &State<AppState>,
    user: SubjectLeaderUser,
) -> Json<ApiResponse> {
    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);

    // Verificar que el SubjectLeader sea el dueño de la asignatura
    match utils::db::update_asignatura(&db.db, asignatura_id, user_id, &asignatura_data.into_inner()).await {
        Ok(_) => Json(ApiResponse::success("Asignatura actualizada exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al actualizar la asignatura: {}", e))),
    }
}

/// Eliminar asignatura - Solo Leaders
#[delete("/asignaturas/delete/<asignatura_id>")]
pub async fn delete_asignatura(
    asignatura_id: i32,
    db: &State<AppState>,
    _leader: LeaderUser,
) -> Json<ApiResponse> {
    match utils::db::delete_asignatura(&db.db, asignatura_id).await {
        Ok(_) => Json(ApiResponse::success("Asignatura eliminada exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al eliminar la asignatura: {}", e))),
    }
}

/// Listar jefes de asignatura - Solo Leaders (para el selector al crear)
#[get("/users/subject_leaders")]
pub async fn list_subject_leaders(
    db: &State<AppState>,
    _leader: LeaderUser,
) -> Json<ApiResponseWithData<Vec<usuarios::Model>>> {
    match utils::db::list_subject_leaders(&db.db).await {
        Ok(leaders) => Json(ApiResponseWithData::success("Jefes de asignatura obtenidos exitosamente".to_string(), leaders)),
        Err(e) => Json(ApiResponseWithData::error(format!("Error al obtener jefes de asignatura: {}", e))),
    }
}
