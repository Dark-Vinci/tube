use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "subscriptions", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "channel_id")]
    pub channel_id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "user_id")]
    pub user_id: Uuid,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "created_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime,

    #[sea_orm(column_type = "Timestamp", column_name = "updated_at", nullable)]
    pub updated_at: Option<DateTime>,

    #[sea_orm(column_type = "Timestamp", column_name = "deleted_at", nullable)]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
