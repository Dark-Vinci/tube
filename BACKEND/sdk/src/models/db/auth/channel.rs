use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "channels", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", column_name = "name", unique, index)]
    pub name: String,
    
    #[sea_orm(column_type = "Text", column_name = "owner")]
    pub owner: Uuid,

    #[sea_orm(
        column_type = "Boolean",
        column_name = "is_active",
        default_value = true
    )]
    pub is_active: bool,

    #[sea_orm(column_type = "Text", column_name = "description", nullable)]
    pub description: Option<String>,

    #[sea_orm(column_type = "Uuid", column_name = "user_id")]
    pub user_id: Uuid,
    
    pub country: String,
    
    pub keywords: Vec<String>,

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

    #[sea_orm(column_type = "DateTime", column_name = "description", nullable)]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    
    #[sea_orm(has_many = "super::channel_users::Entity")]
    Users,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::channel_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
