
use async_trait::async_trait;

#[async_trait]
pub(crate) trait SignIn {
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
}

pub trait Application: SignIn + SignUp {}
