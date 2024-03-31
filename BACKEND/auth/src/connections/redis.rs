use {
    crate::config::config::Config,
    fred::{
        clients::RedisPool,
        error::RedisError,
        interfaces::ClientLike,
        types::{Builder, ConnectHandle, RedisConfig},
    },
    tokio_async_drop::tokio_async_drop,
    tracing::{debug, error},
};

#[derive(Debug)]
pub struct Redis {
    handle: ConnectHandle,
    pub client: RedisPool,
}

impl Drop for Redis {
    fn drop(&mut self) {
        tokio_async_drop!({
            self.client.quit().await.unwrap();
            self.handle.abort()
        });
    }
}

impl Redis {
    #[tracing::instrument(skip(c), name = "Redis::connect")]
    pub async fn connect(c: &Config) -> Result<Self, RedisError> {
        let connection_string = format!(
            "redis://{0}:{1}@{2}:{3}/{4}",
            c.redis_username, c.redis_password, c.redis_host, c.redis_port, c.redis_name
        );

        let conf = RedisConfig::from_url(connection_string.as_str());

        if let Err(e) = conf {
            error!(error = e.to_string(), "Could not parse redis url");

            return Err(e);
        }

        let conf = conf.unwrap();

        let client = Builder::from_config(conf).build_pool(c.redis_pool_size);

        if let Err(e) = client {
            error!(error = e.to_string(), "Could not create a redis builder");

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
}
