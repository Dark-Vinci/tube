use sdk::models::db::auth::ban::Entity as Ban;
use sea_orm::{DbBackend, Schema};

fn main() {
    let db_postgres = DbBackend::Postgres;
    let schema = Schema::new(db_postgres);

    let a = db_postgres.build(&schema.create_table_from_entity(Ban));

    println!("{a}");
}