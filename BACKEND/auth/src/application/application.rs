use crate::application::traits::Application;
use crate::config::config::Config;
use crate::connections::db::DBConnection;
use crate::connections::redis::Redis;
use crate::downstream::downstream::DownStream;

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub downstream: DownStream,
    pub db: DBConnection,
    pub redis: Redis,
}

impl App {
    pub fn new(
        c: Config,
        ds: DownStream,
        db: DBConnection,
        r: Redis,
    ) -> Self {
        Self {
            config: c,
            downstream: ds,
            redis: r,
            db,
        }
    }
}

impl Application for App {}
