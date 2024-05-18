use {
    sdk::models::db::auth::report::Entity as Ban,
    sea_orm::{DbBackend, Schema},
};

fn main() {
    let db_postgres = DbBackend::Sqlite;
    let schema = Schema::new(db_postgres);

    let a = db_postgres.build(&schema.create_table_from_entity(Ban)).sql;

    println!("{a}");
}
