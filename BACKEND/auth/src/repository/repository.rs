use {
    super::{
        ban::BanRepo, channel::ChannelRepo, report::ReportRepo, session::SessionRepo,
        short::ShortRepo, users::UserRepo,
    },
    crate::connections::db::DBConnection,
    std::sync::Arc,
};

#[derive(Debug)]
pub struct Repo {
    pub user: UserRepo,
    pub short: ShortRepo,
    pub session: SessionRepo,
    pub channel: ChannelRepo,
    pub report: ReportRepo,
    pub ban: BanRepo,
}

impl Repo {
    pub fn new(d: DBConnection) -> Self {
        let user = UserRepo::new(Arc::clone(&d.0));
        let short = ShortRepo::new(Arc::clone(&d.0));
        let session = SessionRepo::new(Arc::clone(&d.0));
        let channel = ChannelRepo::new(Arc::clone(&d.0));
        let report = ReportRepo::new(Arc::clone(&d.0));
        let ban = BanRepo::new(Arc::clone(&d.0));

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
