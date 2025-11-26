//! `SeaORM` Entity for balances table
//! Stores teaching load balances with JSONB subjects

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "balances")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub academic_year: String,
    pub period: String,
    pub academic_year_text: String,
    pub start_date: Date,
    pub weeks: i32,
    #[sea_orm(column_type = "JsonBinary")]
    pub subjects: Json,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::usuarios::Entity",
        from = "Column::UserId",
        to = "super::usuarios::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Usuarios,
}

impl Related<super::usuarios::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usuarios.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
