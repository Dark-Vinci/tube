use sea_orm_migration::{MigrationTrait, MigratorTrait};

use super::m20240116_025623_search;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240116_025623_search::Migration)]
    }
}
