use std::env;

use sdk::constants::helper::{
    APP_NAME, AUTH_DB_NAME, AUTH_NAME, AUTH_PORT, DB_HOST,
    DB_PASSWORD, DB_PORT, DB_URL, DB_USERNAME, REACTION_URL,
    REDIS_HOST, REDIS_NAME, REDIS_PASSWORD, REDIS_USERNAME,
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
}

impl Config {
    pub fn new() -> Self {
        Self {
            app_name: env::var(APP_NAME).unwrap(),
            reaction_url: env::var(REACTION_URL).unwrap(),
            posts_url: env::var(DB_URL).unwrap(),
            db_host: env::var(DB_HOST).unwrap(),
            db_port: env::var(DB_PORT).unwrap(),
            db_username: env::var(DB_USERNAME).unwrap(),
            db_password: env::var(DB_PASSWORD).unwrap(),
            db_name: env::var(AUTH_DB_NAME).unwrap(),
            redis_name: env::var(REDIS_NAME).unwrap(),
            redis_password: env::var(REDIS_PASSWORD).unwrap(),
            redis_username: env::var(REDIS_USERNAME).unwrap(),
            redis_host: env::var(REDIS_HOST).unwrap(),
            redis_port: env::var(REDIS_HOST).unwrap(),
            app_port: env::var(AUTH_PORT).unwrap(),
            service_name: env::var(AUTH_NAME).unwrap(),
            redis_pool_size: env::var("REDIS_POOL_SIZE").unwrap_or("8".to_string()).parse::<usize>().unwrap(),
        }
    }
}
