use sea_orm::{DatabaseConnection, Database};

use crate::usuarios;

pub async fn establish_connection() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL no encontrada en el archivo .env");
    println!("🔗 Conectando a la base de datos...");
    let db = Database::connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");

    println!("✅ Conectado a la base de datos exitosamente");
    db
}

pub async fn create_user(db: &DatabaseConnection, name: &str, email: &str, password: &str, role: &str) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    use bcrypt::{hash, DEFAULT_COST};

    let hashed_password = hash(password, DEFAULT_COST).expect("Error al hashear la contraseña");

    let new_user = usuarios::ActiveModel {
        name: Set(name.to_string()),
        email: Set(email.to_string()),
        token: Set(hashed_password),
        role: Set(Some(role.to_string())),
        ..Default::default()
    };

    new_user.insert(db).await?;
    Ok(())
}