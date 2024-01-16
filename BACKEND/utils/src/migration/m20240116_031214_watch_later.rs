use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WatchLater::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WatchLater::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WatchLater::VideoOrPlaylistId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchLater::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchLater::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WatchLater::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(WatchLater::DeletedAt)
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
                Table::drop().table(WatchLater::Table).to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum WatchLater {
    Table,
    Id,
    UserId,
    VideoOrPlaylistId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
