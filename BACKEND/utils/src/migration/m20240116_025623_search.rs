use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Search::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Search::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Search::Content)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Search::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Search::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Search::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Search::DeletedAt)
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
            .drop_table(Table::drop().table(Search::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Search {
    Table,
    Id,
    Content,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
