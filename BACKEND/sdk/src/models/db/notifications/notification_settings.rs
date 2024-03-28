use {sea_orm::prelude::*};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "notification_settings", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "created_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_type = "Timestamp",
        column_name = "updated_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub updated_at: DateTime,

    #[sea_orm(column_type = "Timestamp", column_name = "deleted_at", nullable)]
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
