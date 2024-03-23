use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "blocks", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        column_name = "id",
        column_type = "Uuid",
        auto_increment = false,
        primary_key
    )]
    pub id: Uuid,

    #[sea_orm(column_name = "user_id", column_type = "Uuid")]
    pub user_id: Uuid,

    #[sea_orm(column_name = "channel_id", column_type = "Uuid")]
    pub channel_id: Uuid,

    #[sea_orm(
        column_name = "created_at",
        column_type = "DateTime",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_name = "updated_at",
        column_type = "DateTime",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub updated_at: DateTime,

    #[sea_orm(
        column_name = "deleted_at",
        column_type = "DateTime",
        nullable
    )]
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, DeriveRelation, EnumIter)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
