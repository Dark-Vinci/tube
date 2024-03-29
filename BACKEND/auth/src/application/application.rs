use crate::{
    application::traits::Application, config::config::Config,
    connections::connections::Connections, downstream::downstream::DownStream,
};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub downstream: DownStream,
    pub connections: Connections,
}

impl App {
    pub fn new(c: Config, ds: DownStream, conn: Connections) -> Self {
        Self {
            config: c,
            downstream: ds,
            connections: conn,
        }
    }
}

impl Application for App {}
