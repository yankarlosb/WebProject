use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
//use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;
use std::net::SocketAddr;

// Clave secreta para firmar los tokens (en producción, debe estar en variables de entorno)
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET").unwrap()
});

// Estructura de los claims del JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,    // Subject (user ID)
    pub role: String,   // Rol del usuario
    pub ip: String,     // IP del cliente
    pub exp: usize,     // Expiration time (timestamp)
    pub iat: usize,     // Issued at (timestamp)
}

impl Claims {
    /// Crea un nuevo claim con una expiración de 24 horas
    pub fn new(user_id: i32, role: String, remote_addr: Option<SocketAddr>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        let client_ip = remote_addr
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Claims {
            sub: user_id.to_string(),
            role,
            ip: client_ip,
            iat: now,
            exp: now + 86400, // 24 horas = 86400 segundos
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

    if token_data.claims.ip != remote_addr
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string()) {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    println!("Token decodificado correctamente: {:?}", token_data.claims);

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
        
        // SOLO obtener de cookie HttpOnly
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

pub struct User(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Primero verificamos que esté autenticado
        let auth_user = match request.guard::<AuthenticatedUser>().await {
            Outcome::Success(user) => user,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        // Luego verificamos que tenga un rol válido
        if auth_user.0.role == "subjectLeader"
            || auth_user.0.role == "user"
            || auth_user.0.role == "admin" 
            || auth_user.0.role == "leader"{
            Outcome::Success(User(auth_user.0))
        } else {
            Outcome::Error((Status::Forbidden, ()))
        }
    }
}

/// Guardián que valida que el usuario sea administrador
pub struct AdminUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Primero verificamos que esté autenticado
        let auth_user = match request.guard::<AuthenticatedUser>().await {
            Outcome::Success(user) => user,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        // Verificar que el rol sea "admin"
        if auth_user.0.role == "admin" {
            Outcome::Success(AdminUser(auth_user.0))
        } else {
            // No es admin, denegar acceso
            Outcome::Error((Status::Forbidden, ()))
        }
    }
}

pub struct LeaderUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LeaderUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Primero verificamos que esté autenticado
        let auth_user = match request.guard::<AuthenticatedUser>().await {
            Outcome::Success(user) => user,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        // Verificar que el rol sea "leader"
        if auth_user.0.role == "leader" {
            Outcome::Success(LeaderUser(auth_user.0))
        } else {
            // No es leader, denegar acceso
            Outcome::Error((Status::Forbidden, ()))
        }
    }
}

pub struct SubjectLeaderUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SubjectLeaderUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Primero verificamos que esté autenticado
        let auth_user = match request.guard::<AuthenticatedUser>().await {
            Outcome::Success(user) => user,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        // Verificar que el rol sea "subjectLeader"
        if auth_user.0.role == "subjectLeader" {
            Outcome::Success(SubjectLeaderUser(auth_user.0))
        } else {
            // No es subjectLeader, denegar acceso
            Outcome::Error((Status::Forbidden, ()))
        }
    }
}

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
    pub name: String,
    pub email: String,
    pub role: String,
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
