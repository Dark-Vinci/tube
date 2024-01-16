use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WatchLaters::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WatchLaters::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(
                            WatchLaters::VideoOrPlaylistId,
                        )
                        .uuid()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchLaters::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchLaters::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchLaters::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(WatchLaters::DeletedAt)
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
                Table::drop().table(WatchLaters::Table).to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum WatchLaters {
    Table,
    Id,
    UserId,
    VideoOrPlaylistId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
