use crate::utils::jwt::{AdminUser, AuthenticatedUser};
use crate::*;
use crate::types::{ApiResponse, ApiResponseWithData};
use crate::usuarios;
use rocket::{post, get};
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
    let user_name = &new_user.user_name;
    let name = &new_user.name;
    let email = &new_user.email;
    let password = &new_user.password;
    let role = &new_user.role;

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
    
    match utils::db::update_profile(&db.db, user_id, &profile_data.name, &profile_data.email).await {
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
    
    match utils::db::change_user_password(&db.db, user_id, &password_data.new_password).await {
        Ok(_) => Json(ApiResponse::success("Contraseña cambiada exitosamente".to_string())),
        Err(e) => Json(ApiResponse::error(format!("Error al cambiar la contraseña: {}", e))),
    }
}
