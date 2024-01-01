
#[derive(Debug)]
pub struct Config {
    pub db: String,
    pub redis: String,
    pub app: String,
    pub others: String,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            db: "".to_string(),
            redis: "".to_string(),
            app: "".to_string(),
            others: "".to_string(),
        }
    }
}
