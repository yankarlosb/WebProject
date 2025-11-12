use crate::utils::jwt::AdminUser;
use crate::*;
use crate::types::{ApiResponse, ApiResponseWithData};
use crate::usuarios;
use rocket::{post, get};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
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
    let name = &new_user.name;
    let email = &new_user.email;
    let password = &new_user.password;
    let role = &new_user.role;

    match utils::db::create_user(&db.db, name, email, password, role).await {
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
