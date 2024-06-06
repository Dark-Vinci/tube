use {
    crate::config::config::Config,
    fred::{
        clients::RedisPool,
        error::RedisError,
        interfaces::{ClientLike, KeysInterface},
        types::{
            Builder, ConnectHandle, Expiration, RedisConfig, RedisValue, SetOptions,
        },
    },
    sdk::errors::general::ConnectionError,
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

#[async_trait::async_trait]
pub trait RedisTrait {
    async fn get(&self, key: &str) -> Result<RedisValue, RedisError>;
    async fn set(
        &self,
        key: &str,
        v: Vec<u8>,
        expir: Option<Expiration>,
        opt: Option<SetOptions>,
        get: bool,
    ) -> Result<RedisValue, RedisError>;
}

#[async_trait::async_trait]
impl RedisTrait for Redis {
    async fn get(&self, key: &str) -> Result<RedisValue, RedisError> {
        return self.client.get(key).await;
    }

    async fn set(
        &self,
        key: &str,
        value: Vec<u8>,
        expire: Option<Expiration>,
        options: Option<SetOptions>,
        get: bool,
    ) -> Result<RedisValue, RedisError> {
        return self.client.set(key, value, expire, options, get).await;
    }
}

impl Redis {
    #[tracing::instrument(skip(c), name = "Redis::connect")]
    pub async fn connect(
        c: &Config,
    ) -> Result<Box<dyn RedisTrait + Send + Sync>, ConnectionError> {
        let connection_string = format!(
            "redis://{0}:{1}@{2}:{3}/{4}",
            c.redis_username, c.redis_password, c.redis_host, c.redis_port, c.redis_name
        );

        let conf = RedisConfig::from_url(connection_string.as_str());

        if let Err(e) = conf {
            error!(error = e.to_string(), "Could not parse redis url");

            return Err(ConnectionError::Redis(e));
        }

        let conf = conf.unwrap();

        let client = Builder::from_config(conf).build_pool(c.redis_pool_size);

        if let Err(e) = client {
            error!(error = e.to_string(), "Could not create a redis builder");

            return Err(ConnectionError::Redis(e));
        }

        let client = client.unwrap();

        let connection_task = client.connect();

        client
            .wait_for_connect()
            .await
            .expect("TODO: panic message");

        debug!("FRED successfully connected to the DB");

        Ok(Box::new(Self {
            handle: connection_task,
            client,
        }))
    }
}
