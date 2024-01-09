use std::env;

use sdk::constants::helper::{
    DB_URL, 
    REACTION_URL,
    DB_HOST,
    DB_USERNAME,
    REDIS_PASSWORD,
    REDIS_HOST,
    REDIS_USERNAME,
    REDIS_NAME,
    DB_PASSWORD,
    DB_PORT,
    DB_NAME,
    AUTH_PORT,
};

#[derive(Debug)]
pub struct Config {
    pub db: String,
    pub redis: String,
    pub app: String,
    pub others: String,
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
    pub app_port: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            db: env::var("db").unwrap(),
            redis: env::var("redis").unwrap(),
            app: env::var("app").unwrap(),
            others: env::var("others").unwrap(),
            reaction_url: env::var(REACTION_URL).unwrap(),
            posts_url: env::var(DB_URL).unwrap(),
            db_host: env::var(DB_HOST).unwrap(),
            db_port: env::var(DB_PORT).unwrap(),
            db_username: env::var(DB_USERNAME).unwrap(),
            db_password: env::var(DB_PASSWORD).unwrap(),
            db_name: env::var(DB_NAME).unwrap(),
            redis_name: env::var(REDIS_NAME).unwrap(),
            redis_password: env::var(REDIS_PASSWORD).unwrap(),
            redis_username: env::var(REDIS_USERNAME).unwrap(),
            redis_host: env::var(REDIS_HOST).unwrap(),
            app_port: env::var(AUTH_PORT).unwrap(),
        }
    }
}
