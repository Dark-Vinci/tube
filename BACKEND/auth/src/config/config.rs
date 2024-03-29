use {
    sdk::constants::helper::{
        APP_NAME, AUTH_DB_NAME, AUTH_NAME, AUTH_PORT, DB_HOST, DB_PASSWORD, DB_PORT,
        DB_URL, DB_USERNAME, DEFAULT_DB_AUTH_VALUE, DEFAULT_DB_HOST_VALUE,
        DEFAULT_DB_PASSWORD_VALUE, DEFAULT_DB_PORT_VALUE, DEFAULT_DB_USERNAME_VALUE,
        DEFAULT_REDIS_CONNECTION_POOL, FALSE, IS_PRODUCTION, RABBITMQ_HOST,
        RABBITMQ_PASSWORD, RABBITMQ_PORT, RABBITMQ_USERNAME, REACTION_URL, REDIS_HOST,
        REDIS_NAME, REDIS_PASSWORD, REDIS_POOL_SIZE, REDIS_USERNAME,
    },
    std::env,
};

#[derive(Debug)]
pub struct Config {
    pub app_name: String,
    pub reaction_url: String,
    pub posts_url: String,
    pub db_host: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
    pub redis_name: String,
    pub redis_password: String,
    pub redis_username: String,
    pub redis_host: String,
    pub redis_port: String,
    pub app_port: String,
    pub service_name: String,
    pub redis_pool_size: usize,
    pub is_production: bool,
    pub rabbitmq_username: String,
    pub rabbitmq_password: String,
    pub rabbitmq_host: String,
    pub rabbitmq_port: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            app_name: env::var(APP_NAME).unwrap_or_default(),
            reaction_url: env::var(REACTION_URL).unwrap_or_default(),
            posts_url: env::var(DB_URL).unwrap_or_default(),
            db_host: env::var(DB_HOST).unwrap_or(DEFAULT_DB_HOST_VALUE.into()),
            db_port: env::var(DB_PORT).unwrap_or(DEFAULT_DB_PORT_VALUE.into()),
            db_username: env::var(DB_USERNAME)
                .unwrap_or(DEFAULT_DB_USERNAME_VALUE.into()),
            db_password: env::var(DB_PASSWORD)
                .unwrap_or(DEFAULT_DB_PASSWORD_VALUE.into()),
            db_name: env::var(AUTH_DB_NAME).unwrap_or(DEFAULT_DB_AUTH_VALUE.into()),
            redis_name: env::var(REDIS_NAME).unwrap_or_default(),
            redis_password: env::var(REDIS_PASSWORD).unwrap_or("".into()),
            redis_username: env::var(REDIS_USERNAME).unwrap_or_default(),
            redis_host: env::var(REDIS_HOST).unwrap_or("localhost".into()),
            redis_port: env::var(REDIS_HOST).unwrap_or("6309".into()),
            app_port: env::var(AUTH_PORT).unwrap_or("5050".into()),
            service_name: env::var(AUTH_NAME).unwrap_or_default(),
            is_production: env::var(IS_PRODUCTION)
                .unwrap_or(FALSE.to_string())
                .parse::<bool>()
                .unwrap(),
            redis_pool_size: env::var(REDIS_POOL_SIZE)
                .unwrap_or(DEFAULT_REDIS_CONNECTION_POOL.to_string())
                .parse::<usize>()
                .unwrap(),
            rabbitmq_username: env::var(RABBITMQ_USERNAME).unwrap_or("tomiwa".into()),
            rabbitmq_password: env::var(RABBITMQ_PASSWORD).unwrap_or("tomiwa".into()),
            rabbitmq_host: env::var(RABBITMQ_HOST).unwrap_or("localhost".into()),
            rabbitmq_port: env::var(RABBITMQ_PORT).unwrap_or("5672".into()),
        }
    }
}
