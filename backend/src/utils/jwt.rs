use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
//use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicBool, Ordering};
use once_cell::sync::Lazy;
use std::net::SocketAddr;

// Clave secreta para firmar los tokens (en producción, debe estar en variables de entorno)
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET").unwrap()
});

// Configuración global de validación de IP (actualizable en runtime)
pub static REQUIRE_IP_VALIDATION: AtomicBool = AtomicBool::new(true);

/// Actualiza la configuración de validación de IP
pub fn set_ip_validation(require: bool) {
    REQUIRE_IP_VALIDATION.store(require, Ordering::Relaxed);
}

/// Obtiene si la validación de IP está habilitada
pub fn get_ip_validation() -> bool {
    REQUIRE_IP_VALIDATION.load(Ordering::Relaxed)
}

// Estructura de los claims del JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // Subject (user ID)
    pub user_name: String, // Username para login
    pub name: String,      // Nombre completo
    pub email: String,     // Email del usuario
    pub role: String,      // Rol del usuario
    pub ip: String,        // IP del cliente
    pub exp: usize,        // Expiration time (timestamp)
    pub iat: usize,        // Issued at (timestamp)
}

/// Default token expiration in hours
pub const DEFAULT_TOKEN_EXPIRATION_HOURS: u64 = 3;

impl Claims {
    pub fn new(
        user_id: i32, 
        user_name: String,
        name: String,
        email: String,
        role: String, 
        remote_addr: Option<SocketAddr>,
        expiration_hours: u64,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        let client_ip = remote_addr
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let expiration_secs = expiration_hours * 3600;

        Claims {
            sub: user_id.to_string(),
            user_name,
            name,
            email,
            role,
            ip: client_ip,
            iat: now,
            exp: now + expiration_secs as usize,
        }
    }
}

/// Genera un token JWT a partir de los claims
pub fn create_jwt(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

/// Decodifica y valida un token JWT
pub fn decode_jwt(token: &str, remote_addr: Option<SocketAddr>) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &validation,
    )?;

    // Solo validar IP si está habilitado en la configuración
    if get_ip_validation() {
        let client_ip = remote_addr
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        if token_data.claims.ip != client_ip {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }
    }

    Ok(token_data.claims)
}

// ============================================================================
// GUARDIANES DE AUTENTICACIÓN
// ============================================================================

/// Guardián que valida que el usuario esté autenticado
pub struct AuthenticatedUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        
        let token = cookies.get("jwt_token")
            .map(|c| c.value().to_string());

        match token {
            Some(token) => match decode_jwt(&token, request.remote()) {
                Ok(claims) => Outcome::Success(AuthenticatedUser(claims)),
                Err(_) => Outcome::Error((Status::Unauthorized, ())),
            },
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

/// Helper function to check if a user has a specific role
fn has_role(auth_user: &AuthenticatedUser, allowed_roles: &[&str]) -> bool {
    allowed_roles.contains(&auth_user.0.role.as_str())
}

/// Macro to generate role-based request guards
/// Reduces code duplication by generating the boilerplate for each role guard
macro_rules! impl_role_guard {
    ($guard_name:ident, [$($role:expr),+]) => {
        pub struct $guard_name(pub Claims);

        #[rocket::async_trait]
        impl<'r> FromRequest<'r> for $guard_name {
            type Error = ();

            async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                let auth_user = match request.guard::<AuthenticatedUser>().await {
                    Outcome::Success(user) => user,
                    Outcome::Error(e) => return Outcome::Error(e),
                    Outcome::Forward(f) => return Outcome::Forward(f),
                };

                if has_role(&auth_user, &[$($role),+]) {
                    Outcome::Success($guard_name(auth_user.0))
                } else {
                    Outcome::Error((Status::Forbidden, ()))
                }
            }
        }
    };
}

// Generate role guards using the macro
// User guard - accepts any authenticated user with a valid role
impl_role_guard!(User, ["admin", "leader", "subjectLeader", "user"]);

// Admin guard - only allows admin users
impl_role_guard!(AdminUser, ["admin"]);

// Leader guard - only allows leader users
impl_role_guard!(LeaderUser, ["leader"]);

// SubjectLeader guard - only allows subject leader users
impl_role_guard!(SubjectLeaderUser, ["subjectLeader"]);

// LeaderOrSubjectLeader guard - allows leader or subject leader users
impl_role_guard!(LeaderOrSubjectLeaderUser, ["leader", "subjectLeader"]);

// RESPUESTAS JSON PARA AUTENTICACIÓN
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<UserInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub user_name: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub must_change_password: bool,
}

impl LoginResponse {
    pub fn success(message: String, user: UserInfo) -> Self {
        LoginResponse {
            success: true,
            message,
            user: Some(user),
        }
    }

    pub fn error(message: String) -> Self {
        LoginResponse {
            success: false,
            message,
            user: None,
        }
    }
}
