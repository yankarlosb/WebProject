use crate::*;
use crate::routes::login::ApiResponse;
use rocket::post;
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
) -> Json<ApiResponse> {
    let name = &new_user.name;
    let email = &new_user.email;
    let password = &new_user.password;
    let role = &new_user.role;

    match utils::db::create_user(&db.db, name, email, password, role).await {
        Ok(_) => Json(ApiResponse {
            message: "Usuario creado exitosamente".to_string(),
            alert: "success".to_string(),
        }),
        Err(e) => Json(ApiResponse {
            message: format!("Error al crear el usuario: {}", e),
            alert: "error".to_string(),
        }),
    }
}