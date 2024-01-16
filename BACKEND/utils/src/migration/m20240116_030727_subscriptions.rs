use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subscriptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscriptions::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Subscriptions::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Subscriptions::ChannelId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Subscriptions::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Subscriptions::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Subscriptions::DeletedAt)
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
                Table::drop().table(Subscriptions::Table).to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Subscriptions {
    Table,
    Id,
    ChannelId,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
