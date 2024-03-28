use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// usecase
// comment
// post
// bio
// ....

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "links", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_name = "id",
        column_type = "Uuid",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(column_name = "new_value", column_type = "Text")]
    pub actual_value: String,

    #[sea_orm(column_name = "new_value", column_type = "Text")]
    pub new_value: String,

    #[sea_orm(column_name = "channel_id", column_type = "Uuid")]
    pub channel_id: Uuid,

    #[sea_orm(column_name = "post_id", column_type = "Uuid")]
    pub post_id: Uuid,

    pub post_type: String,

    pub usecase_id: Uuid,

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

    #[sea_orm(column_type = "Timestamp", column_name = "deleted_at", nullable)]
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to="super::"
    // )]
    // UseCase
}

impl ActiveModelBehavior for ActiveModel {}
