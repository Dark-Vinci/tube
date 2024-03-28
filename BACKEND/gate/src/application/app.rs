use crate::{
    application::traits::Application, config::config::Config,
    downstream::downstream::DownStream,
};

#[allow(dead_code)]
pub struct App {
    config: Config,
    downstream: DownStream,
}

impl App {
    pub fn new(c: Config, d: DownStream) -> Self {
        Self {
            config: c,
            downstream: d,
        }
    }
}

impl Application for App {}
