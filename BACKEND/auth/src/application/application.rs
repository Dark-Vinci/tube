use {
    crate::{
        application::traits::Application,
        config::config::Config,
        connections::{
            db::DBConnection,
            rabbit::{Rabbit, RabbitTrait},
            redis::{Redis, RedisTrait},
        },
        downstream::downstream::{DownStream, DownstreamBehaviour},
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
    pub db: DBConnection,
    // trait objects
    pub downstream: Box<dyn DownstreamBehaviour + Send + Sync>,
    pub rabbit: Box<dyn RabbitTrait + Send + Sync>,
    pub redis: Box<dyn RedisTrait + Sync + Send>,
    pub user_repo: Box<dyn UserRepository + Sync + Send>,
    pub session_repo: Box<dyn SessionRepository + Sync + Send>,
    pub report_repo: Box<dyn ReportRepository + Sync + Send>,
    pub channel_repo: Box<dyn ChannelRepository + Sync + Send>,
    pub ban_repo: Box<dyn BanRepository + Sync + Send>,
    pub short_repo: Box<dyn ShortRepository + Sync + Send>,
}

impl App {
    pub async fn new(c: Config) -> Result<Self, E> {
        let (db, redis, rabbit, downstream) = join!(
            DBConnection::open(&c),
            Redis::connect(&c),
            Rabbit::new(&c),
            DownStream::new(&c)
        );

        let db = db?;
        let redis = redis?;
        let rabbit = rabbit?;
        let downstream = downstream?;

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
            downstream,
            user_repo: user,
            session_repo: session,
            report_repo: report,
            channel_repo: channel,
            ban_repo: ban,
            short_repo: short,
            rabbit,
            redis,
            db,
        })
    }
}

impl Application for App {}
