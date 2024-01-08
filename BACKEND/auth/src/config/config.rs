
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
            db: "".to_string(),
            redis: "".to_string(),
            app: "".to_string(),
            others: "".to_string(),
            reaction_url: "".to_string(),
            posts_url: "".to_string(),
            db_host: "".to_string(),
            db_port: "".to_string(),
            db_username: "".to_string(),
            db_password: "".to_string(),
            db_name: "".to_string(),
            redis_name: "".to_string(),
            redis_password: "".to_string(),
            redis_username: "".to_string(),
            redis_host: "".to_string(),
            app_port: "50551".to_string(),
        }
    }
}
