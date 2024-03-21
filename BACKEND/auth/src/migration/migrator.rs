use {
    super::{m20240108_000001_create_cake_table, m20240115_184133_migrationk},
    sea_orm_migration::{MigrationTrait, MigratorTrait},
};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240108_000001_create_cake_table::Migration),
            Box::new(m20240115_184133_migrationk::Migration),
        ]
    }
}
