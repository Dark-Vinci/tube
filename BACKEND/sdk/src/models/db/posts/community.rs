use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum ContentType {
    #[sea_orm(string_value = "Subscription")]
    JustString,

    #[sea_orm(string_value = "Images")]
    Images,

    #[sea_orm(string_value = "Poll")]
    Poll,
}

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "community_posts", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "channel_id", not_null)]
    pub channel_id: Uuid,

    #[sea_orm(column_name = "content", not_null)]
    pub content_type: ContentType,

    #[sea_orm(column_type = "Text", column_name = "content", not_null)]
    pub content: String,

    #[sea_orm(column_type = "Uuid", column_name = "poll_id", nullable)]
    pub poll_id: Option<Uuid>,

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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::images::Entity")]
    Images,
}

impl Related<super::images::Entity> for Relation {
    fn to() -> RelationDef {
        Relation::Images.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
