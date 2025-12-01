// Re-exportar las bibliotecas principales;
use rocket::Build;
pub use rocket;
pub use rocket::routes;
pub use rocket::catchers;

pub use rocket::fs::{FileServer, NamedFile};
pub use rocket::http::Status;
pub use rocket::response::Redirect;
pub use rocket::serde::json::Json;
use rocket::Rocket;
pub use rocket::State;
pub use rocket::form::FromForm;
pub use sea_orm::{Database, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
pub use bcrypt;
pub use dotenvy;
pub use serde;

// Exportar los módulos de entidades
pub mod database;
pub use database::prelude::*;
pub mod utils;
pub mod routes;
pub mod types;

// Re-exportar los módulos específicos de entidades para facilitar el acceso
pub use database::{asignaturas, usuarios};
pub use utils::cors::CORS;
pub use utils::rate_limiter::RateLimiter;


// Importar las rutas para usar en el macro routes!
use routes::login::{
    login_json,
    logout,
    verify_auth,
    unauthorized,
    forbidden
};

use routes::manager::{
    create_user,
    delete_user,
    list_users,
    modify_user,
    update_profile,
    change_password,
    create_asignatura,
    list_asignaturas,
    update_asignatura,
    delete_asignatura,
    list_subject_leaders
};

use routes::balance::{
    list_balances,
    get_balance,
    create_balance,
    update_balance,
    delete_balance
};

use routes::audit::{
    list_audit_logs,
    list_security_logs,
    get_audit_stats
};

pub struct AppState {
    pub db: DatabaseConnection,
    pub rate_limiter: RateLimiter,
}

pub async fn run() -> Rocket<Build> {
    let db = utils::db::establish_connection().await;
    let rate_limiter = RateLimiter::new();
    
    let frontend_path = if cfg!(debug_assertions) {
        "../frontend/src"
    } else {
        "../frontend/dist"
    };

    rocket::build()
        .manage(AppState { db, rate_limiter })
        .attach(CORS)
        .mount("/api", routes![
            login_json,
            create_user,
            delete_user,
            list_users,
            modify_user,
            update_profile,
            change_password,
            logout,
            verify_auth,
            create_asignatura,
            list_asignaturas,
            update_asignatura,
            delete_asignatura,
            list_subject_leaders,
            // Rutas de balances
            list_balances,
            get_balance,
            create_balance,
            update_balance,
            delete_balance,
            // Rutas de auditoría
            list_audit_logs,
            list_security_logs,
            get_audit_stats,
        ])
        .register("/", catchers![unauthorized, forbidden])
        .mount("/", FileServer::from(frontend_path))
}