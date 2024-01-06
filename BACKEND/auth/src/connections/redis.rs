use fred::clients::RedisClient;
use fred::interfaces::ClientLike;
use fred::types::{Builder, ConnectHandle, RedisConfig};

use crate::config::config::Config;

pub struct Redis {
    handle: ConnectHandle,
    pub client: RedisClient,
}

impl Redis {
    pub async fn connect(c: &Config) -> Result<Self, String> {
        let connection_string = format!("redis://{0}:{1}@{2}:6379/{3}", c.redis_username, c.redis_password,c.redis_name, c.db_name);

        let conf = RedisConfig::from_url(connection_string.as_str());

        if let Err(e) = conf {
            Err(e.to_string())
        }

        let conf = conf.unwrap();

        let client = Builder::from_config(conf).build();

        if let Err(e) = client {
            Err(e.to_string())
        }

        let client = client.unwrap();

        let connection_task = client.connect();

        client.wait_for_connect().await.expect("TODO: panic message");

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
