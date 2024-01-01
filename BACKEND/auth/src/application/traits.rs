
use async_trait::async_trait;

#[async::trait]
pub(crate) trait SignIn {
    async fn in_with_google(&self) -> ();
    async fn in_with_email(&self) -> ();
}

#[async::trait]
pub trait SignUp {
    async fn up_with_google(&self) -> ();
    async fn up_with_email(&self) -> ();
}
