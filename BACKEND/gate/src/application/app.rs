use crate::application::traits::Application;
use crate::config::config::Config;

#[derive(Debug)]
pub struct App {
    config: Config,
    downstream: String,
}

impl App {
    pub fn new(c: Config) -> Self {
        Self{
            config: c,
            downstream: "".into(),
        }
    }
}

impl Application for App {}