use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "email_pushes", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_name = "id",
        column_type = "Uuid",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", column_name = "content")]
    pub content: String,

    #[sea_orm(column_name = "user_id", column_type = "Uuid")]
    pub user_id: Uuid,

    #[sea_orm(
        column_type = "Boolean",
        column_name = "sent",
        default_value = "false"
    )]
    pub sent: bool,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "created_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "updated_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub updated_at: DateTime,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "deleted_at",
        nullable
    )]
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation, Copy)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
