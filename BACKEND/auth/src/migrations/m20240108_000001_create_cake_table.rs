use sea_orm::sea_query::Expr;
use sea_orm::DbErr;
use sea_orm_migration::prelude::{
    ColumnDef, DeriveIden, DeriveMigrationName, Table,
};
use sea_orm_migration::{MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[tonic::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::FirstName)
                            .string()
                            .not_null()
                            .comment("First name of the user"),
                    )
                    .col(
                        ColumnDef::new(Users::LastName)
                            .string()
                            .not_null()
                            .comment("Last name of the user"),
                    )
                    .col(
                        ColumnDef::new(Users::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Users::ChannelId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Users::ChannelId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::DateOfBirth)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::ShortId).uuid().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    DateOfBirth,
    CreatedAt,
    UpdatedAt,
    IsActive,
    ChannelId,
    ShortId,
}
