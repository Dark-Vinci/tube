use {
    crate::{
        application::traits::Application,
        config::config::Config,
        connections::{db::DBConnection, rabbit::Rabbit, redis::Redis},
        downstream::downstream::DownStream,
        repository::{
            ban::BanRepository, channel::ChannelRepository, report::ReportRepository,
            repository::Repo, session::SessionRepository, short::ShortRepository,
            users::UserRepository,
        },
    },
    sdk::constants::types::E,
    tokio::join,
};

pub struct App {
    pub config: Config,
    pub downstream: DownStream,
    pub rabbit: Rabbit,
    pub redis: Redis,
    // db repositories
    pub user_repo: Box<dyn UserRepository + Sync + Send + 'static>,
    pub session_repo: Box<dyn SessionRepository + Sync + Send + 'static>,
    pub report_repo: Box<dyn ReportRepository + Sync + Send + 'static>,
    pub channel_repo: Box<dyn ChannelRepository + Sync + Send + 'static>,
    pub ban_repo: Box<dyn BanRepository + Sync + Send + 'static>,
    pub short_repo: Box<dyn ShortRepository + Sync + Send + 'static>,
}

impl App {
    pub async fn new(c: Config, ds: DownStream) -> Result<Self, E> {
        let (db, redis, rabbit) =
            join!(DBConnection::open(&c), Redis::connect(&c), Rabbit::new(&c));

        let db = db?;
        let redis = redis?;
        let rabbit = rabbit?;

        let Repo {
            user,
            short,
            session,
            channel,
            report,
            ban,
        } = Repo::new(&db);

        Ok(Self {
            config: c,
            downstream: ds,
            user_repo: user,
            session_repo: session,
            report_repo: report,
            channel_repo: channel,
            ban_repo: ban,
            short_repo: short,
            rabbit,
            redis,
        })
    }
}

impl Application for App {}
