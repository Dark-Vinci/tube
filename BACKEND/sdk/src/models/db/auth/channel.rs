use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "channels", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "name",
        unique,
        index
    )]
    pub name: String,

    #[sea_orm(
        column_type = "Boolean",
        column_name = "is_active",
        default_value = false
    )]
    pub is_active: bool,

    #[sea_orm(column_type = "Timestamp", column_name = "created_at")]
    pub created_at: DateTime,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "updated_at",
        nullable
    )]
    pub updated_at: Option<DateTime>,

    #[sea_orm(
        column_type = "Text",
        column_name = "description",
        nullable
    )]
    pub deleted_at: Option<DateTime>,

    #[sea_orm(column_type = "Text", column_name = "description")]
    pub description: Option<String>,

    #[sea_orm(column_type = "Uuid")]
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
