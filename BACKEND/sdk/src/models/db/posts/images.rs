use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum UsedFor {
    #[sea_orm(string_value = "Subscription")]
    Cover,

    #[sea_orm(string_value = "Profile")]
    Profile,

    #[sea_orm(string_value = "CommunityPost")]
    CommunityPost,

    #[sea_orm(string_value = "ThumbNail")]
    ThumbNail,
}

#[derive(
    Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "images", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", column_name = "url")]
    pub url: String,

    #[sea_orm(
        column_type = "Uuid",
        column_name = "channel_id",
        nullable
    )]
    pub channel_id: Option<Uuid>,

    #[sea_orm(
        column_type = "Uuid",
        column_name = "user_id",
        nullable
    )]
    pub user_id: Option<Uuid>,

    #[sea_orm(column_name = "used_for")]
    pub used_for: UsedFor,

    #[sea_orm(column_name = "index", default_value = 0)]
    pub index: i32,

    pub usecase_id: Option<Uuid>,

    #[sea_orm(column_type = "Timestamp", column_name = "created_at")]
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
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::community::Entity",
        from = "Column::UsecaseId",
        to = "super::community::Column::Id"
    )]
    CommunityPosts,
}

impl Related<super::community::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CommunityPosts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
