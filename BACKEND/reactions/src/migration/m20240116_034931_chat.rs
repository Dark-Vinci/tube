use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chat::Id)
                            .integer()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Chat::VideoId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Chat::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Chat::Content)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Chat::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Chat::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Chat::DeletedAt)
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
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Chat {
    Table,
    Id,
    VideoId,
    Content,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
