use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
//use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

// Clave secreta para firmar los tokens (en producción, debe estar en variables de entorno)
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET").unwrap()
});

// Estructura de los claims del JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // Subject (user ID)
    pub email: String, // Email del usuario
    pub name: String,  // Nombre del usuario
    pub role: String,  // Rol del usuario
    pub exp: usize,    // Expiration time (timestamp)
    pub iat: usize,    // Issued at (timestamp)
}

impl Claims {
    /// Crea un nuevo claim con una expiración de 24 horas
    pub fn new(user_id: i32, name: String, email: String, role: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Claims {
            sub: user_id.to_string(),
            name,
            email,
            role,
            iat: now,
            exp: now + 86400, // 24 horas = 86400 segundos
        }
    }

    /// Crea un token con una expiración personalizada en segundos
    pub fn with_expiration(
        user_id: i32,
        email: String,
        name: String,
        role: String,
        expiration_secs: usize,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Claims {
            sub: user_id.to_string(),
            email,
            name,
            role,
            iat: now,
            exp: now + expiration_secs,
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
pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

// ============================================================================
// GUARDIANES DE AUTENTICACIÓN
// ============================================================================

/// Guardián que valida que el usuario esté autenticado
/// Extrae el token del header Authorization: Bearer <token> O de la cookie Auth
pub struct AuthenticatedUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        
        // SOLO obtener de cookie HttpOnly
        let token = cookies.get("jwt_token")  // Nombre específico
            .map(|c| c.value().to_string());

        match token {
            Some(token) => match decode_jwt(&token) {
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
    pub token: Option<String>,
    pub user: Option<UserInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

impl LoginResponse {
    pub fn success(token: String, claims: &Claims) -> Self {
        LoginResponse {
            success: true,
            message: "Login exitoso".to_string(),
            token: Some(token),
            user: Some(UserInfo {
                id: claims.sub.clone(),
                name: claims.name.clone(),
                email: claims.email.clone(),
                role: claims.role.clone(),
            }),
        }
    }

    pub fn error(message: String) -> Self {
        LoginResponse {
            success: false,
            message,
            token: None,
            user: None,
        }
    }
}
