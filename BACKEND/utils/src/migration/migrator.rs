use sea_orm_migration::{MigrationTrait, MigratorTrait};

use super::{
    m20240116_025623_search, m20240116_030727_subscriptions,
    m20240116_031214_watch_later, m20240116_031906_watch_until,
    m20240116_032632_notifications,
};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240116_025623_search::Migration),
            Box::new(m20240116_030727_subscriptions::Migration),
            Box::new(m20240116_031214_watch_later::Migration),
            Box::new(m20240116_031906_watch_until::Migration),
            Box::new(m20240116_032632_notifications::Migration),
        ]
    }
}
