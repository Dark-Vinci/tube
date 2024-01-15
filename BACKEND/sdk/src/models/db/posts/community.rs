use sea_orm::entity::prelude::*;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "community_posts", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "channel_id")]
    pub channel_id: Uuid,

    #[sea_orm(column_type = "Text", column_name = "content")]
    pub content: String,

    #[sea_orm(column_type = "Timestamp", column_name = "created_at", default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime,

    #[sea_orm(column_type = "Timestamp", column_name = "updated_at", nullable)]
    pub updated_at: Option<DateTime>,

    #[sea_orm(column_type = "Timestamp", column_name = "deleted_at", nullable)]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::images::Entity")]
    Images
}

impl Related<super::images::Entity> for Relation {
    fn to() -> RelationDef {
        Relation::Images.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
