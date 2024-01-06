use crate::application::traits::Application;
use crate::config::config::Config;
use crate::downstream::downstream::DownStream;

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub downstream: DownStream,
    pub db: String,
}

impl App {
    pub async fn new() -> Result<Self, String> {
        let conf = Config::new();
        let ds = DownStream::new(&conf).await;

        if let Err(e) = ds {
            return Err(e);
        }

        Ok(Self {
            config: Config::new(),
            downstream: ds.unwrap(),
            db: "db".into(),
        })
    }
}

impl Application for App {}
