
use crate::application::traits::{SignIn, SignUp};
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

pub trait Application: SignIn + SignUp {}

impl Application for App {}
