use crate::connections::db::DBConnection;

use super::ban::BanRepo;
use super::channel::ChannelRepo;
use super::report::ReportRepo;
use super::session::SessionRepo;
use super::short::ShortRepo;
use super::users::UserRepo;

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
        let user = UserRepo::new(&d);
        let short = ShortRepo::new(&d);
        let session = SessionRepo::new(&d);
        let channel = ChannelRepo::new(&d);
        let report = ReportRepo::new(&d);
        let ban = BanRepo::new(&d);

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
