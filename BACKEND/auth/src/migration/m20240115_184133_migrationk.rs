use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Report::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Report::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Report::EntityId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Report::CreatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Report::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Report::DeletedAt)
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
            .drop_table(Table::drop().table(Report::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Report {
    Table,
    Id,
    EntityId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
