use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, Serialize, DeriveEntityModel, Deserialize,
)]
#[sea_orm(table_name = "polls", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id", column_type = "Uuid")]
    pub id: Uuid,

    pub community_post_id: Uuid,

    pub post_type: String,
}

enum Relation {}

impl ActiveModelBehavior for Entity {}