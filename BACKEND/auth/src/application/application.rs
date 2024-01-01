
use crate::application::traits::{Application};
use crate::config::config::Config;

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }
}

impl Application for App {}
