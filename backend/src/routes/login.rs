use crate::utils::jwt::{AuthenticatedUser, Claims, LoginResponse, UserInfo, create_jwt};
use crate::*;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::content;
use rocket::time::Duration;
use rocket::{catch, get, post};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct LoginJson {
    username: String,
    password: String,
}

// Esta ruta maneja el login desde Vue.js
#[post("/login", format = "json", data = "<credentials>")]
pub async fn login_json(
    credentials: Json<LoginJson>,
    db: &State<AppState>,
    cookies: &CookieJar<'_>,
    remote_addr: Option<SocketAddr>,
) -> Json<LoginResponse> {
    let username = &credentials.username;
    let password = &credentials.password;

    // Buscar el usuario en la base de datos
    let entity = match usuarios::Entity::find()
        .filter(usuarios::Column::Name.eq(username))
        .one(&db.db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Json(LoginResponse::error("Usuario no encontrado".to_string()));
        }
        Err(_) => {
            return Json(LoginResponse::error("Error del servidor".to_string()));
        }
    };

    // Verificar la contraseña
    let verify = bcrypt::verify(password, &entity.token).unwrap_or(false);
    if !verify {
        return Json(LoginResponse::error("Contraseña incorrecta".to_string()));
    }

    // Crear los claims del JWT
    let claims = Claims::new(
        entity.id,
        entity.role.clone().unwrap_or_default(),
        remote_addr,
    );

    // Generar el token
    match create_jwt(&claims) {
        Ok(token) => {
            // Establecer cookie HttpOnly con el token (seguridad adicional)
            let mut cookie = Cookie::new("jwt_token", token.clone());
            cookie.set_http_only(false);
            cookie.set_same_site(SameSite::Lax);
            cookie.set_secure(true);
            cookie.set_path("/");
            cookie.set_max_age(Duration::hours(24));
            cookies.add(cookie);

            // Crear información del usuario para la respuesta
            let user_info = UserInfo {
                id: entity.id,
                name: entity.name.clone(),
                email: entity.email.clone(),
                role: entity.role.clone().unwrap_or_else(|| "user".to_string()),
            };

            Json(LoginResponse::success(
                "Login exitoso".to_string(),
                user_info,
            ))
        }
        Err(_) => Json(LoginResponse::error(
            "Error al generar el token".to_string(),
        )),
    }
}

/// Página de balance - Solo usuarios autenticados
#[get("/balance")]
pub async fn balance_page(_user: AuthenticatedUser) -> Option<NamedFile> {
    // El guardián valida automáticamente la cookie JWT
    // Si no está autenticado, devuelve 401 y Rocket maneja el error
    NamedFile::open("../frontend/balance.html").await.ok()
}

/// Logout - Elimina la cookie JWT y redirecciona al login
#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    // Eliminar la cookie JWT
    cookies.remove(Cookie::build("jwt_token"));

    // Redireccionar al login
    Redirect::to("/login")
}

/// Estructura para la respuesta de verificación
#[derive(Serialize)]
pub struct VerifyResponse {
    success: bool,
    authenticated: bool,
}

#[get("/verify")]
pub fn verify_auth(_user: AuthenticatedUser) -> Json<VerifyResponse> {
    Json(VerifyResponse {
        success: true,
        authenticated: true,
    })
}

/// Catcher para error 401 (No autorizado) - Muestra alerta y redirige
#[catch(401)]
pub fn unauthorized() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
        <!DOCTYPE html>
        <html>
        <head><meta charset="UTF-8"></head>
        <body>
            <script>
                alert('⚠️ Por favor, inicie sesión para acceder a esta página.');
                window.location.href = '/login';
            </script>
        </body>
        </html>
    "#,
    )
}
