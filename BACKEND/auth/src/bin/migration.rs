use sea_orm_migration::prelude::*;

use auth::migration::migrator::Migrator;

#[tokio::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
