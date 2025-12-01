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
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    // Filter at database level instead of fetching all and filtering in memory
    let users = usuarios::Entity::find()
        .filter(usuarios::Column::Role.ne("admin"))
        .all(db)
        .await?;
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

/// Update profile - updates only name and email directly without redundant query
pub async fn update_profile(
    db: &DatabaseConnection,
    user_id: i32,
    name: &str,
    email: &str,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};
    
    // Get user and update directly in one operation instead of calling change_profile
    let user = usuarios::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Usuario no encontrado".to_string()))?;
    
    let mut user_active: ActiveModel = user.into();
    user_active.name = Set(name.to_string());
    user_active.email = Set(email.to_string());
    user_active.update(db).await?;
    
    Ok(())
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

// ============================================================================
// FUNCIONES DE ASIGNATURAS
// ============================================================================

use crate::asignaturas;
use crate::routes::manager::{CreateAsignaturaRequest, UpdateAsignaturaRequest};
use chrono::Utc;

/// Crear asignatura - busca el subject leader por username y crea con datos por defecto
pub async fn create_asignatura(
    db: &DatabaseConnection,
    data: &CreateAsignaturaRequest,
) -> Result<i32, sea_orm::DbErr> {
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    // Buscar el usuario por user_name con role 'subjectLeader'
    let subject_leader = usuarios::Entity::find()
        .filter(usuarios::Column::UserName.eq(&data.leader_user_name))
        .filter(usuarios::Column::Role.eq("subjectLeader"))
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(format!(
            "No se encontró un jefe de asignatura con username '{}'",
            data.leader_user_name
        )))?;

    // Crear asignatura con datos por defecto
    let new_asignatura = asignaturas::ActiveModel {
        leader_id: Set(subject_leader.id),
        name: Set(data.name.clone()),
        year: Set(data.year.clone()),
        semester: Set(data.semester.clone()),
        c: Set(None),
        cp: Set(None),
        s: Set(None),
        pl: Set(None),
        te: Set(None),
        t: Set(None),
        pp: Set(None),
        ec: Set(None),
        tc: Set(None),
        ef: Set(None),
        hours: Set(0),
        weeks: Set(Some(15)),
        date_start: Set(Utc::now().naive_utc()),
        date_end: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let result = new_asignatura.insert(db).await?;
    Ok(result.id)
}

/// Listar asignaturas - filtra según el rol del usuario
pub async fn list_asignaturas(
    db: &DatabaseConnection,
    user_id: i32,
    role: &str,
) -> Result<Vec<asignaturas::Model>, sea_orm::DbErr> {
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    if role == "leader" {
        // Leaders ven todas las asignaturas
        asignaturas::Entity::find().all(db).await
    } else if role == "subjectLeader" {
        // Subject Leaders solo ven sus asignaturas
        asignaturas::Entity::find()
            .filter(asignaturas::Column::LeaderId.eq(user_id))
            .all(db)
            .await
    } else {
        Ok(vec![])
    }
}

/// Actualizar asignatura - solo si el usuario es el dueño (subject leader)
pub async fn update_asignatura(
    db: &DatabaseConnection,
    asignatura_id: i32,
    user_id: i32,
    data: &UpdateAsignaturaRequest,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    // Buscar la asignatura
    let asignatura = asignaturas::Entity::find_by_id(asignatura_id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Asignatura no encontrada".to_string()))?;

    // Verificar que el usuario sea el dueño
    if asignatura.leader_id != user_id {
        return Err(sea_orm::DbErr::Custom(
            "No tienes permisos para editar esta asignatura".to_string()
        ));
    }

    // Actualizar asignatura
    let mut asignatura_active: asignaturas::ActiveModel = asignatura.into();
    asignatura_active.name = Set(data.name.clone());
    asignatura_active.year = Set(data.year.clone());
    asignatura_active.semester = Set(data.semester.clone());
    asignatura_active.c = Set(data.c);
    asignatura_active.cp = Set(data.cp);
    asignatura_active.s = Set(data.s);
    asignatura_active.pl = Set(data.pl);
    asignatura_active.te = Set(data.te);
    asignatura_active.t = Set(data.t);
    asignatura_active.pp = Set(data.pp);
    asignatura_active.ec = Set(data.ec);
    asignatura_active.tc = Set(data.tc);
    asignatura_active.ef = Set(data.ef);
    asignatura_active.hours = Set(data.hours);
    asignatura_active.weeks = Set(data.weeks);

    asignatura_active.update(db).await?;
    Ok(())
}

/// Eliminar asignatura - solo leaders
pub async fn delete_asignatura(
    db: &DatabaseConnection,
    asignatura_id: i32,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::{EntityTrait, ModelTrait};

    let asignatura = asignaturas::Entity::find_by_id(asignatura_id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Asignatura no encontrada".to_string()))?;

    asignatura.delete(db).await?;
    Ok(())
}

/// Listar jefes de asignatura (role = 'subjectLeader')
pub async fn list_subject_leaders(
    db: &DatabaseConnection,
) -> Result<Vec<usuarios::Model>, sea_orm::DbErr> {
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    usuarios::Entity::find()
        .filter(usuarios::Column::Role.eq("subjectLeader"))
        .all(db)
        .await
}