use crate::application::traits::Application;
use crate::config::config::Config;
use crate::connections::db::DBConnection;
use crate::downstream::downstream::DownStream;

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub downstream: DownStream,
    pub db: DBConnection,
}

impl App {
    pub fn new(c: Config, ds: DownStream, db: DBConnection) -> Self {
        Self {
            config: c,
            downstream: ds,
            db,
        }
    }
}

impl Application for App {}
