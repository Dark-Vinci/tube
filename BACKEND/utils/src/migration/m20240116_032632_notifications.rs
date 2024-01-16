use sea_orm_migration::prelude::*;

use crate::migration::m20240116_031906_watch_until::WatchUntil;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notifications::Id)
                            .integer()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Notifications::Content)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(
                            Notifications::NotificationType,
                        )
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Notifications::PostId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Notifications::ToUserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchUntil::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchUntil::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(WatchUntil::DeletedAt)
                            .timestamp()
                            .null(),
                    )
                    .index(
                        Index::create()
                            .name("user_id_idx")
                            .col(WatchUntil::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop().table(Notifications::Table).to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Notifications {
    Table,
    Id,
    PostId,
    ToUserId,
    Content,
    NotificationType,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
