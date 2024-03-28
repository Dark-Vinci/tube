use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Serialize, DeriveEntityModel, Deserialize)]
#[sea_orm(table_name = "polls", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_name = "id",
        column_type = "Uuid",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(column_name = "title", column_type = "Uuid")]
    pub title: Uuid,

    #[sea_orm(column_name = "channel_id", column_type = "Uuid")]
    pub channel_id: Uuid,

    #[sea_orm(column_name = "option_a")]
    pub option_a: String,

    #[sea_orm(column_name = "option_b")]
    pub option_b: String,

    #[sea_orm(column_name = "option_c", nullable)]
    pub option_c: Option<String>,

    #[sea_orm(column_name = "option_d", nullable)]
    pub option_d: Option<String>,

    #[sea_orm(column_name = "created_at", nullable)]
    pub option_e: Option<String>,

    #[sea_orm(
        column_name = "created_at",
        default_value = "CURRENT_TIMESTAMP",
        column_type = "DateTime"
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_name = "updated_at",
        default_value = "CURRENT_TIMESTAMP",
        column_type = "DateTime"
    )]
    pub updated_at: DateTime,

    #[sea_orm(column_name = "deleted_at", nullable, column_type = "DateTime")]
    pub deleted_at: Option<DateTime>,
}

#[derive(EnumIter, DeriveRelation, Debug, Copy, Clone)]
pub enum Relation {
    #[sea_orm(has_many = "super::votes::Entity")]
    Vote,
}

impl Related<super::votes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Vote.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
