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

pub struct Model {
    pub id: String,
    
    pub user_id: String,
    
    pub device: String,
    
    pub ip_address: String,
    
    pub permission: Permission,
    
    pub location: String,
    
    pub created_at: String,
    
    pub updated_at: String,
    
    pub deleted_at: Option<String>,
    
    pub expires_at: String,
}
