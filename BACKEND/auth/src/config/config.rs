
#[derive(Debug)]
pub struct Config {
    pub db: String,
    pub redis: String,
    pub app: String,
    pub others: String,
    pub reaction_url: String,
    pub posts_url: String,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            db: "".to_string(),
            redis: "".to_string(),
            app: "".to_string(),
            others: "".to_string(),
            reaction_url: "".to_string(),
            posts_url: "".to_string(),
        }
    }
}
