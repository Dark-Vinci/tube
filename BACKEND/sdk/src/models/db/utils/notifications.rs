use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Type {
    #[sea_orm(string_value = "Subscription")]
    Subscription,

    #[sea_orm(string_value = "Comment")]
    Comment,

    #[sea_orm(string_value = "Unsubscribe")]
    Unsubscribe,

    #[sea_orm(string_value = "Like")]
    Like,

    #[sea_orm(string_value = "Post")]
    Post,

    #[sea_orm(string_value = "Stream")]
    Stream,
}

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "notifications", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", column_name = "post_id")]
    pub post_id: Option<Uuid>,

    #[sea_orm(column_name = "type")]
    pub r#type: Type,

    pub content: Option<String>,

    #[sea_orm(column_type = "Uuid", column_name = "to_user_id")]
    pub to_user_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
