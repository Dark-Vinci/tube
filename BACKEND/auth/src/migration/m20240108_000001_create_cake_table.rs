use {
    sea_orm::{
        sea_query::{Expr, Index},
        DbErr,
    },
    sea_orm_migration::{
        prelude::{ColumnDef, DeriveIden, DeriveMigrationName, Table},
        {MigrationTrait, SchemaManager},
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[tonic::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
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
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().null())
                    .col(ColumnDef::new(Users::DeletedAt).timestamp().null())
                    .col(ColumnDef::new(Users::DateOfBirth).date().not_null())
                    .col(ColumnDef::new(Users::Password).text().not_null())
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::ChannelName)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::About).text().default("no about"))
                    .col(ColumnDef::new(Users::ProfileId).uuid().null())
                    .col(ColumnDef::new(Users::BannerId).uuid().null())
                    .col(ColumnDef::new(Users::AuthToken).text().null())
                    .col(ColumnDef::new(Users::AuthProvider).text().null())
                    .col(ColumnDef::new(Users::Socials).json())
                    .index(Index::create().name("email_idx").col(Users::Email))
                    .index(
                        Index::create()
                            .name("channel_name_index")
                            .col(Users::ChannelName),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
    DeletedAt,
    IsActive,
    Email,
    Password,
    ChannelName,
    About,
    Socials,
    ProfileId,
    BannerId,
    AuthToken,
    AuthProvider,
}
