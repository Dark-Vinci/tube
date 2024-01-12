use crate::connections::db::DBConnection;

use super::ban::BanRepo;
use super::channel::ChannelRepo;
use super::report::ReportRepo;
use super::session::SessionRepo;
use super::short::ShortRepo;
use super::users::UserRepo;

#[derive(Debug)]
pub struct Repo<'a> {
    pub user: UserRepo<'a>,
    pub short: ShortRepo<'a>,
    pub session: SessionRepo<'a>,
    pub channel: ChannelRepo<'a>,
    pub report: ReportRepo<'a>,
    pub ban: BanRepo<'a>,
}

impl<'a> Repo<'a> {
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
