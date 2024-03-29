use {
    super::{
        ban::BanRepo, channel::ChannelRepo, report::ReportRepo, session::SessionRepo,
        short::ShortRepo, users::UserRepo,
    },
    crate::connections::db::DBConnection,
    sdk::models::db::auth::{ban, channel, report, session, short, user},
    std::sync::Arc,
    tonic::async_trait,
    uuid::Uuid,
};

#[derive(Debug)]
pub struct Repo {
    user: UserRepo,
    short: ShortRepo,
    session: SessionRepo,
    channel: ChannelRepo,
    report: ReportRepo,
    ban: BanRepo,
}

#[async_trait]
pub trait Repository {
    async fn create_user(
        &self,
        request_id: Uuid,
        b: user::Model,
    ) -> Result<user::Model, String>;
    async fn create_short(
        &self,
        request_id: Uuid,
        b: short::Model,
    ) -> Result<short::Model, String>;
    async fn create_session(
        &self,
        request_id: Uuid,
        b: session::Model,
    ) -> Result<session::Model, String>;
    async fn create_channel(
        &self,
        request_id: Uuid,
        b: channel::Model,
    ) -> Result<channel::Model, String>;
    async fn create_ban(
        &self,
        request_id: Uuid,
        b: ban::Model,
    ) -> Result<ban::Model, String>;
    async fn create_report(
        &self,
        request_id: Uuid,
        b: report::Model,
    ) -> Result<report::Model, String>;
}

#[async_trait]
impl Repository for Repo {
    async fn create_user(
        &self,
        request_id: Uuid,
        b: user::Model,
    ) -> Result<user::Model, String> {
        return self.user.create(request_id, b).await;
    }

    async fn create_short(
        &self,
        request_id: Uuid,
        b: short::Model,
    ) -> Result<short::Model, String> {
        return self.short.create(request_id, b).await;
    }

    async fn create_session(
        &self,
        request_id: Uuid,
        b: session::Model,
    ) -> Result<session::Model, String> {
        return self.session.create(request_id, b).await;
    }

    async fn create_channel(
        &self,
        request_id: Uuid,
        b: channel::Model,
    ) -> Result<channel::Model, String> {
        return self.channel.create(request_id, b).await;
    }

    async fn create_ban(
        &self,
        request_id: Uuid,
        b: ban::Model,
    ) -> Result<ban::Model, String> {
        return self.ban.create(request_id, b).await;
    }

    async fn create_report(
        &self,
        request_id: Uuid,
        b: report::Model,
    ) -> Result<report::Model, String> {
        return self.report.create(request_id, b).await;
    }
}

impl Repo {
    pub fn new(d: &DBConnection) -> Self {
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
