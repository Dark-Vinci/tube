use {
    super::{
        ban::{BanRepo, BanRepository},
        channel::{ChannelRepo, ChannelRepository},
        report::{ReportRepo, ReportRepository},
        session::{SessionRepo, SessionRepository},
        short::{ShortRepo, ShortRepository},
        users::{UserRepo, UserRepository},
    },
    crate::connections::db::DBConnection,
    std::sync::Arc,
};

pub struct Repo {
    pub user: Box<dyn UserRepository + Sync + Send>,
    pub short: Box<dyn ShortRepository + Sync + Send>,
    pub session: Box<dyn SessionRepository + Sync + Send>,
    pub channel: Box<dyn ChannelRepository + Sync + Send>,
    pub report: Box<dyn ReportRepository + Sync + Send>,
    pub ban: Box<dyn BanRepository + Sync + Send>,
}

impl Repo {
    pub fn new(d: &DBConnection) -> Self {
        let user = UserRepo::create(Arc::clone(&d.0));
        let short = ShortRepo::create(Arc::clone(&d.0));
        let session = SessionRepo::create(Arc::clone(&d.0));
        let channel = ChannelRepo::create(Arc::clone(&d.0));
        let report = ReportRepo::create(Arc::clone(&d.0));
        let ban = BanRepo::create(Arc::clone(&d.0));

        Self {
            user,
            short,
            session,
            channel,
            report,
            ban,
        }
    }
}
