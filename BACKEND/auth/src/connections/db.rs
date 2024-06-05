use {
    crate::{config::config::Config, migration::migrator::Migrator},
    sdk::{
        errors::general::ConnectionError, helpers::util,
        models::schema::general::Environment,
    },
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
    sea_orm_migration::MigratorTrait,
    std::{sync::Arc, time::Duration},
    tracing::{debug, error},
    uuid::Uuid,
};

#[derive(Debug, Clone, Default)]
pub struct DBConnection(pub Arc<DatabaseConnection>);

// impl Drop for DBConnection {
//     fn drop(&mut self) {
//         block_in_place(|| {
//             Handle::current().block_on(async { self.close().await.unwrap() });
//         });
//     }
// }

impl DBConnection {
    pub async fn open(c: &Config) -> Result<Self, ConnectionError> {
        let connection_string = if c.environment == Environment::Testing {
            let id = Uuid::new_v4();
            debug!("sqlite_test_id: {}", id.to_string());

            util::sqlite_test_document(id)
        } else {
            format!(
                "postgres://{0}:{1}@{2}:{3}/{4}",
                c.db_username, c.db_password, c.db_host, c.db_port, c.db_name
            )
        };

        let mut opt = ConnectOptions::new(connection_string);

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(opt).await;

        if let Err(e) = db {
            error!(e = e.to_string(), "DB Connection error");
            return Err(ConnectionError::DB(e));
        }

        let db = db.unwrap();

        if c.environment != Environment::Production {
            // running for the first time;
            // Migrator::install(&db).await.unwrap();

            Migrator::up(&db, None).await.unwrap();
        }

        debug!("connected to the DB");

        Ok(Self(Arc::new(db)))
    }

    // async fn close(&self) -> Result<(), DbErr> {
    //     let a = self.0.clone();

    //     let conn = Arc::try_unwrap(a).unwrap();

    //     conn.close().await
    // }

    pub fn get_connection(&self) -> Arc<DatabaseConnection> {
        self.0.clone()
    }
}
