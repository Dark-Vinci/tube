use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "videos", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "channel_id", indexed)]
    pub channel_id: Uuid,

    #[sea_orm(column_name = "url", column_type = "Text")]
    pub url: String,

    #[sea_orm(column_type = "Uuid", column_name = "playlist_id", indexed)]
    pub playlist_id: Option<Uuid>,

    #[sea_orm(column_name = "title")]
    pub title: String,

    #[sea_orm(column_name = "description", column_type = "Text")]
    pub description: String,

    #[sea_orm(column_name = "video_length_in_seconds", default_value = 0)]
    pub video_length_in_seconds: u32,

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
    #[sea_orm(
        belongs_to = "super::playlist::Entity",
        from = "Column::PlaylistId",
        to = "super::playlist::Column::Id"
    )]
    Playlist,
}

impl Related<super::playlist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Playlist.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
