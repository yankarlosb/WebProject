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

// Re-exportar los módulos específicos de entidades para facilitar el acceso
pub use database::{asignaturas, usuarios};
pub use utils::cors::CORS;


// Importar las rutas para usar en el macro routes!
use routes::login::{
    login_json,
    balance_page,
    logout,
    verify_auth,
    unauthorized
};

use routes::manager::{
    create_user,
    delete_user,
    list_users,
    modify_user
};

pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn run() -> Rocket<Build> {
    let db = utils::db::establish_connection().await;
    
    let frontend_path = if cfg!(debug_assertions) {
        "../frontend/src"
    } else {
        "../frontend/dist"
    };

    rocket::build()
        .manage(AppState { db })
        .attach(CORS)
        .mount("/api", routes![
            login_json,
            create_user,
            delete_user,
            list_users,
            modify_user,
            logout,
            verify_auth,
            balance_page,
        ])
        .register("/", catchers![unauthorized])
        .mount("/", FileServer::from(frontend_path))
}