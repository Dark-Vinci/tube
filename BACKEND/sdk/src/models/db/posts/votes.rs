use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, Serialize, DeriveEntityModel, Deserialize,
)]
#[sea_orm(table_name = "votes", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id", column_type = "Uuid")]
    pub id: Uuid,

    #[sea_orm(column_name = "channel_id", column_type = "Uuid")]
    pub channel_id: Uuid,

    #[sea_orm(column_name = "user_id", column_type = "Uuid")]
    pub user_id: Uuid,

    #[sea_orm(column_name = "poll_id", column_type = "Uuid")]
    pub poll_id: Uuid,

    #[sea_orm(
        column_name = "option_a_vote_count",
        column_type = "BigInteger"
    )]
    pub option_a_vote_count: i64,

    #[sea_orm(
        column_name = "option_b_vote_count",
        column_type = "BigInteger"
    )]
    pub option_b_vote_count: i64,

    #[sea_orm(
        column_name = "option_c_vote_count",
        column_type = "BigInteger"
    )]
    pub option_c_vote_count: Option<i64>,

    #[sea_orm(
        column_name = "option_d_vote_count",
        column_type = "BigInteger"
    )]
    pub option_d_vote_count: Option<i64>,

    #[sea_orm(
        column_name = "option_e_vote_count",
        column_type = "BigInteger"
    )]
    pub option_e_vote_count: Option<i64>,

    #[sea_orm(
        column_name = "total_votes",
        column_type = "BigInteger"
    )]
    pub total_votes: i64,

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

    #[sea_orm(
        column_name = "deleted_at",
        nullable,
        column_type = "DateTime"
    )]
    pub deleted_at: Option<DateTime>,
}

#[derive(EnumIter, DeriveRelation, Debug, Copy, Clone)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::poll::Entity",
        from = "Column::PollId",
        to = "super::poll::Column::Id"
    )]
    Poll,
}

impl Related<super::poll::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Poll.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
