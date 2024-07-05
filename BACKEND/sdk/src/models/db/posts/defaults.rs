use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};


#[derive(Clone, EnumIter, DeriveActiveEnum, PartialEq, Debug, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Visibility {
    #[sea_orm(string_value = "public")]
    Public,

    #[sea_orm(string_value = "private")]
    Private,

    #[sea_orm(string_value = "unlisted")]
    Unlisted,
}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "post_defaults", schema_name = "public")]
struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
        auto_increment = false
    )]
    pub id: Uuid,
    
    #[sea_orm(
        column_name = "title",
    )]
    pub title: String,
    
    #[sea_orm(
        column_name = "language"
    )]
    pub language: String,
    
    
    pub visibility: Visibility,
    
    #[sea_orm(
        column_type = "Json",
        column_name = "tags"
    )]
    pub tags: serde_json::Value,
    
    #[sea_orm(
        column_type = "Uuid",
        column_name = "channel_id",
    )]
    pub channel_id: Uuid,

    #[sea_orm(column_name = "lincense")]
    pub lincense: String,
    
    #[sea_orm(column_name = "category")]
    pub category: String,
    
    #[sea_orm(column_name = "title_and_description_language")]
    pub title_and_description_language: String,
    
    #[sea_orm(
        column_type = "Boolean",
        column_name = "activate_comments",
        default_value = true
    )]
    pub activate_comments: bool,
    
    #[sea_orm(
        column_type = "Boolean",
        column_name = "comments_moderation",
        default_value = false
    )]
    pub comments_moderation: bool,
    
    #[sea_orm(
        column_type = "Boolean",
        column_name = "show_how_many_viewers_like_video",
        default_value = true
    )]
    pub show_how_many_viewers_like_video: bool,
    
    #[sea_orm(
        column_type = "DateTime",
        column_name = "created_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_type = "DateTime",
        column_name = "updated_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub updated_at: DateTime,

    #[sea_orm(
        column_type = "DateTime", 
        column_name = "description", 
        nullable
    )]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::db::auth::channel::Entity", 
        from = "Column::ChannelId",
        to = "crate::models::db::auth::channel::Column::Id"
    )]
    Channel,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<crate::models::db::auth::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}
