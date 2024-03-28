use {
    sea_orm::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "reports", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_name = "id",
        column_type = "Uuid",
        auto_increment = false
    )]
    pub id: Uuid,

    // type
    pub usecase: String,

    #[sea_orm(column_name = "usecase_id", column_type = "Uuid")]
    pub usecase_id: Uuid,

    #[sea_orm(column_name = "channel_id", column_type = "Uuid")]
    pub channel_id: Uuid,

    #[sea_orm(column_name = "user_id", column_type = "Uuid")]
    pub user_id: Uuid,

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

    #[sea_orm(column_name = "deleted_at", column_type = "DateTime", nullable)]
    pub deleted_at: Option<DateTime>,
}

// community post, video, short
#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
