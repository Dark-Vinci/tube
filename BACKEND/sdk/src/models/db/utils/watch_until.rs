use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "watched_untils", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "user_id", indexed)]
    pub user_id: Uuid,

    #[sea_orm(primary_key, column_type = "Uuid", column_name = "video_id")]
    pub video_id: Uuid,

    #[sea_orm(column_name = "last_second_watch", default_value = 0)]
    pub last_second_watch: u32,

    #[sea_orm(column_name = "video_length_in_seconds", default_value = 0)]
    pub video_length_in_seconds: u32,

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
