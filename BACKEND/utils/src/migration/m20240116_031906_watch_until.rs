use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WatchUntil::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WatchUntil::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WatchUntil::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchUntil::VideoId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchUntil::LastSecondWatch)
                            .double()
                            .default(0.00)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(
                            WatchUntil::VideoLengthInSeconds,
                        )
                        .double()
                        .default(0.00)
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
                Table::drop().table(WatchUntil::Table).to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum WatchUntil {
    Table,
    Id,
    UserId,
    VideoId,
    LastSecondWatch,
    VideoLengthInSeconds,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
