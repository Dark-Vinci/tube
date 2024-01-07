use crate::application::traits::Application;
use crate::config::config::Config;
use crate::connections::redis::Redis;
use crate::downstream::downstream::DownStream;
use crate::repository::repository::Repo;

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub downstream: DownStream,
    pub repo: Repo,
    pub redis: Redis,
}

impl App {
    pub fn new(
        c: Config,
        ds: DownStream,
        rp: Repo,
        r: Redis,
    ) -> Self {
        Self {
            config: c,
            downstream: ds,
            redis: r,
            repo: rp,
        }
    }
}

impl Application for App {}
