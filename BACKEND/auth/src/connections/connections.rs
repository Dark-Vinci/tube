use {
    super::db::DBConnection,
    crate::{
        config::config::Config,
        connections::{rabbit::Rabbit, redis::Redis},
        repository::repository::Repo,
    },
    sdk::constants::types::E,
    tokio::join,
};

#[derive(Debug)]
pub struct Connections {
    pub redis: Redis,
    pub repo: Repo,
    pub rabbit: Rabbit,
}

impl Connections {
    pub async fn new(config: &Config) -> Result<Self, E> {
        let (db, redis, rabbit) = join!(
            DBConnection::open(config),
            Redis::connect(config),
            Rabbit::new(config)
        );

        let db = db?;
        let redis = redis?;
        let rabbit = rabbit?;

        // using DB connection, bootstrap repository
        let repo = Repo::new(&db);

        Ok(Self {
            redis,
            rabbit,
            repo,
        })
    }
}
