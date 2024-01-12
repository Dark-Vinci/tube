use crate::application::traits::Application;
use crate::config::config::Config;
use crate::connections::redis::Redis;
use crate::downstream::downstream::DownStream;
use crate::repository::repository::Repo;

#[derive(Debug)]
pub struct App<'a> {
    pub config: Config<'a>,
    pub downstream: DownStream,
    pub repo: Repo<'a>,
    pub redis: Redis,
}

impl<'a> App<'a> {
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
