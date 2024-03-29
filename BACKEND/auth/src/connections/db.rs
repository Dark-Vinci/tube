use {
    crate::config::config::Config,
    sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr},
    std::time::Duration,
    tracing::{debug, error},
};

#[derive(Debug, Clone)]
pub struct DBConnection(pub DatabaseConnection);

impl DBConnection {
    pub async fn open(c: &Config) -> Result<Self, String> {
        let connection_string = format!(
            "postgres://{0}:{1}@{2}/{3}",
            c.db_username, c.db_password, c.db_host, c.db_name
        );

        let mut opt = ConnectOptions::new(connection_string);

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("my_schema");

        let db = Database::connect(opt).await;

        if let Err(e) = db {
            error!(e = e.to_string(), "DB Connection error");
            return Err(e.to_string());
        }

        debug!("CONNECTED TO POSTGRES DB");

        Ok(Self(db.unwrap()))
    }

    pub async fn close(self) -> Result<(), DbErr> {
        self.0.close().await
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.0
    }
}
