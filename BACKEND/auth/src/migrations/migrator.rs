use sea_orm_migration::{MigrationTrait, MigratorTrait};

use crate::migrations::m20240108_000001_create_cake_table;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240108_000001_create_cake_table::Migration)]
    }
}
