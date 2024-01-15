use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "dislikes", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "user_id")]
    pub user_id: Uuid,

    #[sea_orm(
        column_type = "Uuid",
        column_name = "video_id",
        indexed
    )]
    pub video_id: Uuid,

    #[sea_orm(
        column_type = "Uuid",
        column_name = "comment_id",
        nullable
    )]
    pub comment_id: Option<Uuid>,

    #[sea_orm(column_type = "Boolean", column_name = "dislike")]
    pub dislike: bool,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "created_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "updated_at",
        nullable
    )]
    pub updated_at: Option<DateTime>,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "deleted_at",
        nullable
    )]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
