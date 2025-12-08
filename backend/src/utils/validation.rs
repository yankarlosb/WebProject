//! Utilidades de validación para prevenir inyecciones y datos maliciosos

use regex::Regex;
use once_cell::sync::Lazy;

// Patrones de validación compilados una sola vez
static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9_]+$").unwrap()
});

static NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-ZáéíóúÁÉÍÓÚüÜñÑ\s]+$").unwrap()
});

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

static SUBJECT_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9áéíóúÁÉÍÓÚüÜñÑ\s\-\.\(\)]+$").unwrap()
});

/// Valida que un username solo contenga caracteres permitidos (alfanumérico y _)
pub fn is_valid_username(username: &str) -> bool {
    if username.len() < 3 || username.len() > 50 {
        return false;
    }
    USERNAME_REGEX.is_match(username)
}

/// Valida que un nombre solo contenga letras y espacios
pub fn is_valid_name(name: &str) -> bool {
    let trimmed = name.trim();
    if trimmed.len() < 2 || trimmed.len() > 100 {
        return false;
    }
    NAME_REGEX.is_match(trimmed)
}

/// Valida formato de email
pub fn is_valid_email(email: &str) -> bool {
    let trimmed = email.trim();
    if trimmed.len() > 254 {
        return false;
    }
    EMAIL_REGEX.is_match(trimmed)
}

/// Valida nombre de asignatura
pub fn is_valid_subject_name(name: &str) -> bool {
    let trimmed = name.trim();
    if trimmed.len() < 2 || trimmed.len() > 200 {
        return false;
    }
    SUBJECT_NAME_REGEX.is_match(trimmed)
}

/// Valida contraseña (mínimo 8 caracteres)
pub fn is_valid_password(password: &str) -> bool {
    if password.len() <= 8 && password.len() >= 128 {
        return false
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_special = password.chars().any(|c| c.is_alphanumeric());
    let has_number = password.chars().any(|c| c.is_numeric());
    
    has_lowercase && has_uppercase && has_special && has_number
}

/// Sanitiza texto removiendo caracteres peligrosos para SQL/XSS
pub fn sanitize_text(input: &str) -> String {
    input
        .chars()
        .filter(|c| !matches!(c, '<' | '>' | '\'' | '"' | '`' | ';' | '\\' | '{' | '}' | '[' | ']' | '|' | '&' | '$'))
        .collect::<String>()
        .trim()
        .to_string()
}

/// Resultado de validación con mensaje de error
pub struct ValidationResult {
    pub valid: bool,
    pub error: Option<String>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self { valid: true, error: None }
    }
    
    pub fn error(msg: &str) -> Self {
        Self { valid: false, error: Some(msg.to_string()) }
    }
}

/// Valida datos de nuevo usuario
pub fn validate_new_user(username: &str, name: &str, email: &str, password: &str) -> ValidationResult {
    if !is_valid_username(username) {
        return ValidationResult::error("Nombre de usuario inválido");
    }
    if !is_valid_name(name) {
        return ValidationResult::error("Nombre inválido");
    }
    if !is_valid_email(email) {
        return ValidationResult::error("Email inválido");
    }
    if !is_valid_password(password) {
        return Json(ApiResponse::error("Contraseña inválida (mínimo 8 caracteres, mayúsculas, minúsculas y caracteres especiales)".to_string()));
    }
    ValidationResult::ok()
}

/// Valida datos de perfil
pub fn validate_profile(name: &str, email: &str) -> ValidationResult {
    if !is_valid_name(name) {
        return ValidationResult::error("Nombre inválido");
    }
    if !is_valid_email(email) {
        return ValidationResult::error("Email inválido");
    }
    ValidationResult::ok()
}

/// Valida nombre de asignatura
pub fn validate_subject(name: &str) -> ValidationResult {
    if !is_valid_subject_name(name) {
        return ValidationResult::error("Nombre de asignatura inválido");
    }
    ValidationResult::ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username() {
        assert!(is_valid_username("usuario123"));
        assert!(is_valid_username("user_name"));
        assert!(!is_valid_username("us")); // muy corto
        assert!(!is_valid_username("user@name")); // caracter inválido
        assert!(!is_valid_username("user name")); // espacio no permitido
    }

    #[test]
    fn test_valid_name() {
        assert!(is_valid_name("Juan Pérez"));
        assert!(is_valid_name("María García López"));
        assert!(!is_valid_name("J")); // muy corto
        assert!(!is_valid_name("Juan123")); // números no permitidos
    }

    #[test]
    fn test_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name@domain.co"));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("@example.com"));
    }

    #[test]
    fn test_valid_subject_name() {
        assert!(is_valid_subject_name("Programación Web I"));
        assert!(is_valid_subject_name("Matemáticas (Nivel 2)"));
        assert!(!is_valid_subject_name("X")); // muy corto
        assert!(!is_valid_subject_name("Test<script>")); // caracteres peligrosos
    }
}
