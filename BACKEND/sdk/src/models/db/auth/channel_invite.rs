use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "channel_invite", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_name="id",
        column_type="Uuid",
    )]
    pub id: Uuid,
    
    #[sea_orm(
        column_name="channel_id",
        column_type="Uuid",
        indexed,
    )]
    pub channel_id: Uuid,
    
    #[sea_orm(
        column_name="invited_by",
        column_type="Uuid",
    )]
    pub invited_by: Uuid,
    
    #[sea_orm(
        column_name="email",
        column_type="String",
    )]
    pub email: String,
    
    #[sea_orm(
        column_name="is_active",
        column_type="Boolean",
        default_value=false,
    )]
    pub is_active: bool,
    
    #[sea_orm(
        column_name="created_at",
        column_type="Timestamp",
        default_value="CURRENT_TIMESTAMP",
    )]
    pub created_at: DateTime,
    
    #[sea_orm(
        column_name="updated_at",
        column_type="Timestamp",
        default_value="CURRENT_TIMESTAMP",
    )]
    pub updated_at: DateTime,
    
    #[sea_orm(
        column_name="deleted_at",
        column_type="Timestamp",
        nullable,
    )]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}
