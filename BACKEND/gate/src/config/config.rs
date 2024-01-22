#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 0,
        }
    }
}
