use sea_orm::{Database, DatabaseConnection};

use crate::usuarios::{self, ActiveModel};

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

pub async fn create_user(
    db: &DatabaseConnection,
    name: &str,
    email: &str,
    password: &str,
    role: &str,
) -> Result<(), sea_orm::DbErr> {
    use bcrypt::{DEFAULT_COST, hash};
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;

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

pub async fn delete_user(db: &DatabaseConnection, user_id: i32) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ColumnTrait;
    use sea_orm::EntityTrait;
    use sea_orm::QueryFilter;

    usuarios::Entity::delete_many()
        .filter(usuarios::Column::Id.eq(user_id))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn list_users(db: &DatabaseConnection) -> Result<Vec<usuarios::Model>, sea_orm::DbErr> {
    use sea_orm::EntityTrait;

    let users = usuarios::Entity::find()
        .all(db)
        .await?
        .iter()
        .filter(|user| user.role != Some("admin".to_string()))
        .cloned()
        .collect::<Vec<_>>();
    Ok(users)
}

/// Modify user data (admin operation - can change role)
pub async fn modify_user(db: &DatabaseConnection, user_id: i32, user_data: &usuarios::Model) -> Result<(), sea_orm::DbErr> {
    update_user_fields(db, user_id, user_data, true).await
}

/// Change user profile (user operation - cannot change role)
pub async fn change_profile(db: &DatabaseConnection, user_id: i32, user_data: usuarios::Model) -> Result<(), sea_orm::DbErr> {
    update_user_fields(db, user_id, &user_data, false).await
}

/// Internal helper to update user fields
async fn update_user_fields(
    db: &DatabaseConnection,
    user_id: i32,
    user_data: &usuarios::Model,
    update_role: bool,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ActiveModelTrait;
    use sea_orm::EntityTrait;
    use sea_orm::Set;

    let mut modified_user: ActiveModel = usuarios::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .expect("No se pudo obtener el usuario")
        .into();

    // Always update name and email
    modified_user.name = Set(user_data.name.clone());
    modified_user.email = Set(user_data.email.clone());

    // Only update role if allowed (admin operation)
    if update_role {
        modified_user.role = Set(user_data.role.clone());
    }

    modified_user.update(db).await?;
    Ok(())
}