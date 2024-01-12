use std::env;

use sdk::constants::helper::{
    APP_NAME, AUTH_DB_NAME, AUTH_NAME, AUTH_PORT, DB_HOST,
    DB_PASSWORD, DB_PORT, DB_URL, DB_USERNAME, REACTION_URL,
    REDIS_HOST, REDIS_NAME, REDIS_PASSWORD, REDIS_USERNAME,
};

#[derive(Debug)]
pub struct Config<'a> {
    pub app_name: &'a str,
    pub reaction_url: &'a str,
    pub posts_url: &'a str,
    pub db_host: &'a str,
    pub db_port: &'a str,
    pub db_username: &'a str,
    pub db_password: &'a str,
    pub db_name: &'a str,
    pub redis_name: &'a str,
    pub redis_password: &'a str,
    pub redis_username: &'a str,
    pub redis_host: &'a str,
    pub redis_port: &'a str,
    pub app_port: &'a str,
    pub service_name: &'a str,
}

impl<'a> Config<'a> {
    pub fn new() -> Self {
        Self {
            app_name: &env::var(APP_NAME).unwrap(),
            reaction_url: &env::var(REACTION_URL).unwrap(),
            posts_url: &env::var(DB_URL).unwrap(),
            db_host: &env::var(DB_HOST).unwrap(),
            db_port: &env::var(DB_PORT).unwrap(),
            db_username: &env::var(DB_USERNAME).unwrap(),
            db_password: &env::var(DB_PASSWORD).unwrap(),
            db_name: &env::var(AUTH_DB_NAME).unwrap(),
            redis_name: &env::var(REDIS_NAME).unwrap(),
            redis_password: &env::var(REDIS_PASSWORD).unwrap(),
            redis_username: &env::var(REDIS_USERNAME).unwrap(),
            redis_host: &env::var(REDIS_HOST).unwrap(),
            redis_port: &env::var(REDIS_HOST).unwrap(),
            app_port: &env::var(AUTH_PORT).unwrap(),
            service_name: &env::var(AUTH_NAME).unwrap(),
        }
    }
}
