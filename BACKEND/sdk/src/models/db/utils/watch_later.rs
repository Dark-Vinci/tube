use sea_orm::entity::prelude::*;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "watch_laters", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    #[sea_orm(
        column_type = "Uuid",
        column_name = "user_id",
        indexed
    )]
    pub user_id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "video_id")]
    pub video_id: Uuid,

    #[sea_orm(column_type = "Timestamp", column_name = "created_at")]
    pub created_at: DateTime,

    #[sea_orm(column_type = "Timestamp", column_name = "updated_at")]
    pub updated_at: Option<DateTime>,

    #[sea_orm(column_type = "Timestamp", column_name = "deleted_at")]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
