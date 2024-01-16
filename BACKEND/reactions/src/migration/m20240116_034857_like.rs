use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Like::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Like::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Like::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Like::PostId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Like::CommentId).uuid().null(),
                    )
                    .col(
                        ColumnDef::new(Like::Like)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Like::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Like::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Like::DeletedAt)
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
            .drop_table(Table::drop().table(Like::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Like {
    Table,
    Id,
    UserId,
    PostId,
    CommentId,
    Like,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
