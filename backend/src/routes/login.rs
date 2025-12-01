use crate::utils::jwt::{AuthenticatedUser, Claims, LoginResponse, UserInfo, create_jwt};
use crate::utils::validation::is_valid_username;
use crate::utils::audit;
use crate::*;
use rocket::http::{Cookie, CookieJar, SameSite};
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
    let username = credentials.username.trim();
    let password = &credentials.password;

    // Obtener IP del cliente
    let client_ip = remote_addr.map(|addr| addr.ip());
    
    // Verificar si la IP está bloqueada por rate limiting (single lock acquisition)
    if let Some(ip) = client_ip {
        let (is_blocked, remaining) = db.rate_limiter.check_block_status(ip);
        if is_blocked {
            return Json(LoginResponse::error(
                if let Some(secs) = remaining {
                    format!("Demasiados intentos fallidos. Intente de nuevo en {} segundos.", secs)
                } else {
                    "Demasiados intentos fallidos. Intente más tarde.".to_string()
                }
            ));
        }
    }

    // Validar formato de username (prevenir inyecciones)
    if !is_valid_username(username) {
        // Registrar intento fallido
        if let Some(ip) = client_ip {
            db.rate_limiter.record_failed_attempt(ip);
            // Registrar en auditoría
            let _ = audit::log_login_failed(&db.db, username, &ip.to_string(), "Usuario inválido").await;
        }
        return Json(LoginResponse::error("Usuario inválido".to_string()));
    }

    // Buscar el usuario en la base de datos
    let entity = match usuarios::Entity::find()
        .filter(usuarios::Column::UserName.eq(username))
        .one(&db.db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Registrar intento fallido
            if let Some(ip) = client_ip {
                db.rate_limiter.record_failed_attempt(ip);
                // Registrar en auditoría
                let _ = audit::log_login_failed(&db.db, username, &ip.to_string(), "Usuario no encontrado").await;
            }
            return Json(LoginResponse::error("Credenciales inválidas".to_string()));
        }
        Err(_) => {
            return Json(LoginResponse::error("Error del servidor".to_string()));
        }
    };

    // Verificar la contraseña
    let verify = bcrypt::verify(password, &entity.token).unwrap_or(false);
    if !verify {
        // Registrar intento fallido
        if let Some(ip) = client_ip {
            let blocked = db.rate_limiter.record_failed_attempt(ip);
            // Registrar en auditoría
            let _ = audit::log_login_failed(&db.db, username, &ip.to_string(), "Contraseña incorrecta").await;
            if blocked {
                return Json(LoginResponse::error(
                    "Demasiados intentos fallidos. Cuenta bloqueada temporalmente.".to_string()
                ));
            }
            let remaining = db.rate_limiter.get_remaining_attempts(ip);
            if remaining <= 2 {
                return Json(LoginResponse::error(
                    format!("Credenciales inválidas. {} intentos restantes.", remaining)
                ));
            }
        }
        return Json(LoginResponse::error("Credenciales inválidas".to_string()));
    }

    // Login exitoso - limpiar intentos fallidos
    if let Some(ip) = client_ip {
        db.rate_limiter.record_success(ip);
        // Periodically cleanup old entries to prevent memory growth
        db.rate_limiter.maybe_cleanup();
    }

    // Crear los claims del JWT con toda la información del usuario
    let claims = Claims::new(
        entity.id,
        entity.user_name.clone(),
        entity.name.clone(),
        entity.email.clone(),
        entity.role.clone().unwrap_or_default(),
        remote_addr,
    );

    // Generar el token
    match create_jwt(&claims) {
        Ok(token) => {
            // Establecer cookie HttpOnly con el token (seguridad adicional)
            let mut cookie = Cookie::new("jwt_token", token.clone());
            cookie.set_http_only(true);
            cookie.set_same_site(SameSite::Lax);
            cookie.set_secure(true);
            cookie.set_path("/");
            cookie.set_max_age(Duration::seconds(10800));
            cookies.add(cookie);

            // Registrar login exitoso en auditoría
            let ip_str = client_ip.map(|ip| ip.to_string()).unwrap_or_else(|| "unknown".to_string());
            let _ = audit::log_login_success(&db.db, entity.id, &entity.user_name, &ip_str).await;

            // Crear información del usuario para la respuesta
            let user_info = UserInfo {
                id: entity.id,
                user_name: entity.user_name.clone(),
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

/// Logout - Elimina la cookie JWT y registra en auditoría
#[post("/logout")]
pub async fn logout(
    cookies: &CookieJar<'_>,
    db: &State<AppState>,
    user: Option<AuthenticatedUser>,
    remote_addr: Option<SocketAddr>,
) -> Redirect {
    // Registrar logout en auditoría si hay usuario
    if let Some(auth_user) = user {
        let user_id = auth_user.0.sub.parse::<i32>().unwrap_or(0);
        let ip_str = remote_addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());
        let _ = audit::log_logout(&db.db, user_id, &auth_user.0.user_name, &ip_str).await;
    }
    
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
    user: Option<UserInfo>,
}

#[get("/verify")]
pub fn verify_auth(user: AuthenticatedUser) -> Json<VerifyResponse> {
    // Extraer los datos del usuario desde los claims del JWT
    let user_info = UserInfo {
        id: user.0.sub.parse().unwrap_or(0),
        user_name: user.0.user_name.clone(),
        name: user.0.name.clone(),
        email: user.0.email.clone(),
        role: user.0.role.clone(),
    };

    Json(VerifyResponse {
        success: true,
        authenticated: true,
        user: Some(user_info),
    })
}

/// Estructura para respuesta de error de autenticación
#[derive(Serialize)]
pub struct UnauthorizedResponse {
    success: bool,
    message: String,
    alert: String,
}

/// Catcher para error 401 (No autorizado)
/// Devuelve JSON para peticiones API, HTML para navegación directa
#[catch(401)]
pub fn unauthorized(req: &rocket::Request<'_>) -> (rocket::http::Status, rocket::serde::json::Value) {
    // Verificar si la petición es a un endpoint de API
    if req.uri().path().as_str().starts_with("/api/") {
        // Devolver JSON para peticiones de API
        (
            rocket::http::Status::Unauthorized,
            rocket::serde::json::json!({
                "success": false,
                "message": "No autorizado. Por favor, inicie sesión.",
                "alert": "error"
            })
        )
    } else {
        // Para navegación directa, devolver un objeto JSON que el cliente puede manejar
        (
            rocket::http::Status::Unauthorized,
            rocket::serde::json::json!({
                "redirect": "/login",
                "message": "Por favor, inicie sesión para acceder a esta página."
            })
        )
    }
}

/// Catcher para error 403 (Prohibido/Forbidden)
/// Devuelve JSON para peticiones API
#[catch(403)]
pub fn forbidden(req: &rocket::Request<'_>) -> (rocket::http::Status, rocket::serde::json::Value) {
    if req.uri().path().as_str().starts_with("/api/") {
        (
            rocket::http::Status::Forbidden,
            rocket::serde::json::json!({
                "success": false,
                "message": "Acceso denegado. No tiene permisos para esta operación.",
                "alert": "error"
            })
        )
    } else {
        (
            rocket::http::Status::Forbidden,
            rocket::serde::json::json!({
                "redirect": "/",
                "message": "No tiene permisos para acceder a esta página."
            })
        )
    }
}
