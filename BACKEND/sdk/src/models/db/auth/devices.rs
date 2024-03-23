use uuid::Uuid;
use sea_orm::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name="devices")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", column_name = "id")]
    pub id: Uuid,

    pub created_at: DateTime,

    pub updated_at: DateTime,

    pub deleted_at: Option<DateTime>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}