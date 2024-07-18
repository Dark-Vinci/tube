use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, EnumIter, DeriveActiveEnum, PartialEq, Debug, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Permission {
    #[sea_orm(string_value = "admin")]
    Admin,

    #[sea_orm(string_value = "channel_id")]
    ChannelAdmin,

    #[sea_orm(string_value = "user")]
    User,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "logins", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_name = "id",
        column_type = "Uuid",
        auto_increment = false
    )]
    pub id: Uuid,
    
    #[sea_orm(
        column_name="user_id",
        column_type="Uuid",
        indexed
    )]
    pub user_id: String,
    
    #[sea_orm(
        column_name="device",
    )]
    pub device: String,
    
    #[sea_orm(
        column_name="ip_address",
    )]
    pub ip_address: String,
    
    pub permission: Permission,
    
    #[sea_orm(
        column_name="location",
    )]
    pub location: String,
    
    #[sea_orm(
        column_type="Text",
        column_name="token",
    )]
    pub token: String,
    
    #[sea_orm(
        column_type="Timestamp",
        column_name="created_at",
        default_value="CURRENT_TIMESTAMP",
    )]
    pub created_at: DateTime,
    
    #[sea_orm(
        column_type="Timestamp",
        column_name="updated_at",
        default_value="CURRENT_TIMESTAMP",
    )]
    pub updated_at: DateTime,
    
    #[sea_orm(
        column_type="Timestamp",
        column_name="updated_at",
        nullable,
    )]
    pub deleted_at: Option<DateTime>,
    
    #[sea_orm(
        column_type="Timestamp",
        column_name="expires_at",
    )]
    pub expires_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel{}