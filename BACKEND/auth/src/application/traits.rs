use tonic::async_trait;

#[async_trait]
pub trait SignIn {
    async fn in_with_google(&self) -> ();
    async fn in_with_email(&self) -> ();
}

#[async_trait]
pub trait SignUp {
    async fn up_with_google(&self) -> ();
    async fn up_with_email(&self) -> ();
}

#[async_trait]
pub trait Subscribe {
    async fn toggle(&self) -> ();
    async fn get_subscribers(&self) -> ();
}

#[async_trait]
pub trait Ban {
    async fn ban_user(&self) -> ();
    async fn ban_channel(&self) -> ();
    async fn ban_short(&self) -> ();
    async fn unban_user(&self) -> ();
    async fn unban_channel(&self) -> ();
    async fn unban_short(&self) -> ();
}

#[async_trait]
pub trait Report {
    async fn report_user(&self) -> ();
    async fn report_channel(&self) -> ();
    async fn report_short(&self) -> ();
    async fn report_channel_video(&self) -> ();
    async fn report_short_video(&self) -> ();
    async fn report_comment(&self) -> ();
}

#[async_trait]
pub trait Short {}

pub trait Application: SignIn + SignUp + Report + Subscribe + Ban + Short {}
