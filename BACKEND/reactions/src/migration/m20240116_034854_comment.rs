use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Comment::CommentId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Comment::CommentType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comment::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comment::PostId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comment::PostId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comment::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comment::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Comment::DeletedAt)
                            .timestamp()
                            .null(),
                    )
                    .index(
                        Index::create()
                            .name("post_id_idx")
                            .col(Comment::PostId),
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
                Table::drop().table(Comment::Table).to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Comment {
    Table,
    Id,
    PostId,
    Content,
    UserId,
    CommentId,
    CommentType,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
