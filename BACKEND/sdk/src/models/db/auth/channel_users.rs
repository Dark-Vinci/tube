use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, EnumIter, DeriveActiveEnum, PartialEq, Debug, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Permission {
    #[sea_orm(string_value = "owner")]
    Owner,

    #[sea_orm(string_value = "viewer")]
    Viewer,

    #[sea_orm(string_value = "editor")]
    Editor,
    
    #[sea_orm(string_value = "manager")]
    Manager,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "channel_invite", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
        auto_increment = false
    )]
    pub id: Uuid,
    
    #[sea_orm(
        column_type = "Uuid",
        column_name = "channel_id"
    )]
    pub channel_id: Uuid,
    
    #[sea_orm(
        column_type = "Uuid",
        column_name = "invited_by"
    )]
    pub invited_by: Uuid,
    
    pub permission: Permission,
    
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
        column_name = "deleted_at", 
        nullable
    )]
    pub deleted_at: Option<DateTime>,
    
    #[sea_orm(
        column_type = "Boolean",
        column_name = "active",
        default_value = true
    )]
    active: bool,
    
    #[sea_orm(
        column_type = "Boolean",
        column_name = "status",
        default_value = false
    )]
    status: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::channel::Entity",
        from = "Column::ChannelId",
        to = "super::channel::Column::Id",
    )]
    Channel,
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
