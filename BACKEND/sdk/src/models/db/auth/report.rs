use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
    )]
    pub id: Uuid,

    #[sea_orm(
        column_type = "Uuid",
        index,
    )]
    pub user_id: String,

    #[sea_orm(
        column_type = "Boolean",
        column_name = "is_active",
        default_value = false,
    )]
    pub is_active: bool,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "created_at",
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_type = "Timestamp",
        nullable,
    )]
    pub expires_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::db::auth::user::Entity",
        from = "Column::UserId",
        to = "crate::models::db::auth::user::Column::Id",
    )]
    User,
}

impl Related<crate::models::db::auth::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

