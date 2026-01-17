use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Obtener el origen permitido de las variables de entorno o usar localhost por defecto
        let allowed_origins = std::env::var("ALLOWED_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:5173".to_string());

        let request_origin = request.headers().get_one("Origin");

        // Lógica para manejar CORS dinámico o wildcard
        let origin_to_allow = if let Some(origin) = request_origin {
            // Si la configuración es "*", permitimos el origen de la petición (para soportar credenciales)
            if allowed_origins == "*" {
                origin
            } 
            // Si hay una lista separada por comas, verificamos si el origen está en la lista
            else if allowed_origins.split(',').any(|o| o.trim() == origin) {
                origin
            } 
            // Si no coincide, devolvemos la configuración tal cual (fallará si no coincide)
            else {
                allowed_origins.as_str()
            }
        } else {
            // Si no hay header Origin (ej. curl), usamos el configurado
            allowed_origins.as_str()
        };

        response.set_header(Header::new("Access-Control-Allow-Origin", origin_to_allow.to_string()));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}