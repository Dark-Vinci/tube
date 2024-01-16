use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dislike::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dislike::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Dislike::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Dislike::PostId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Dislike::CommentId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Dislike::Dislike)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Dislike::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Dislike::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Dislike::DeletedAt)
                            .timestamp()
                            .null(),
                    )
                    .index(
                        Index::create()
                            .name("post_id_idx")
                            .col(Dislike::PostId),
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
                Table::drop().table(Dislike::Table).to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Dislike {
    Table,
    Id,
    UserId,
    PostId,
    CommentId,
    Dislike,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
