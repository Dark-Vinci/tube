use fred::clients::RedisClient;
use fred::error::RedisError;
use fred::interfaces::ClientLike;
use fred::types::{Builder, ConnectHandle, RedisConfig};
use tracing::{debug, error};

use crate::config::config::Config;

#[derive(Debug)]
pub struct Redis {
    handle: ConnectHandle,
    pub client: RedisClient,
}

impl Redis {
    #[tracing::instrument(skip(c), name = "Redis::connect")]
    pub async fn connect(c: &Config) -> Result<Self, RedisError> {
        let connection_string = format!(
            "redis://{0}:{1}@{2}:{3}/{4}",
            c.redis_username,
            c.redis_password,
            c.redis_name,
            c.redis_port,
            c.redis_name
        );

        let conf = RedisConfig::from_url(connection_string.as_str());

        if let Err(e) = conf {
            error!(
                error = e.to_string(),
                "Could not parse redis url"
            );

            return Err(e);
        }

        let conf = conf.unwrap();

        let client = Builder::from_config(conf).build();

        if let Err(e) = client {
            error!(
                error = e.to_string(),
                "Could not create a redis builder"
            );

            return Err(e);
        }

        let client = client.unwrap();

        let connection_task = client.connect();

        client
            .wait_for_connect()
            .await
            .expect("TODO: panic message");

        debug!("FRED successfully connected to the DB");

        Ok(Self {
            handle: connection_task,
            client,
        })
    }

    pub async fn close(&self) {
        // close client connection
        self.client.quit().await.unwrap();

        self.handle.abort();
    }
}
