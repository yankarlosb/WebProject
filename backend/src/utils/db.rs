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
    user_name: &str,
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
        user_name: Set(user_name.to_string()),
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

    // Always update user_name, name and email
    modified_user.user_name = Set(user_data.user_name.clone());
    modified_user.name = Set(user_data.name.clone());
    modified_user.email = Set(user_data.email.clone());

    // Only update role if allowed (admin operation)
    if update_role {
        modified_user.role = Set(user_data.role.clone());
    }

    modified_user.update(db).await?;
    Ok(())
}

/// Update profile - simplified wrapper for change_profile
pub async fn update_profile(
    db: &DatabaseConnection,
    user_id: i32,
    name: &str,
    email: &str,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::EntityTrait;
    
    // Get current user data
    let mut user = usuarios::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Usuario no encontrado".to_string()))?;
    
    // Update only name and email
    user.name = name.to_string();
    user.email = email.to_string();
    
    // Use existing change_profile which won't modify role
    change_profile(db, user_id, user).await
}

/// Change user password - simple update without verification
pub async fn change_user_password(
    db: &DatabaseConnection,
    user_id: i32,
    new_password: &str,
) -> Result<(), sea_orm::DbErr> {
    use bcrypt::{DEFAULT_COST, hash};
    use sea_orm::ActiveModelTrait;
    use sea_orm::EntityTrait;
    use sea_orm::Set;

    // Hash new password
    let hashed_password = hash(new_password, DEFAULT_COST)
        .map_err(|_| sea_orm::DbErr::Custom("Error al hashear contraseña".to_string()))?;

    // Get user and update only password
    let user = usuarios::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Usuario no encontrado".to_string()))?;

    let mut user_active: ActiveModel = user.into();
    user_active.token = Set(hashed_password);
    user_active.update(db).await?;

    Ok(())
}