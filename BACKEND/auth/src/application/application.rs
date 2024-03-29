use crate::{
    application::traits::Application,
    config::config::Config,
    connections::{rabbit::Rabbit, redis::Redis},
    downstream::downstream::DownStream,
    repository::repository::Repo,
};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub downstream: DownStream,
    pub repo: Repo,
    pub redis: Redis,
    pub rabbit: Rabbit,
}

impl App {
    pub fn new(c: Config, ds: DownStream, rp: Repo, r: Redis, rb: Rabbit) -> Self {
        Self {
            config: c,
            downstream: ds,
            redis: r,
            repo: rp,
            rabbit: rb,
        }
    }
}

impl Application for App {}
