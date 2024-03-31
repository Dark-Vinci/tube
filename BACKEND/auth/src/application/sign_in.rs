use {
    super::{application::App, traits::SignIn},
    tonic::async_trait,
};

#[async_trait]
impl SignIn for App {
    async fn in_with_google(&self) -> () {
        // self.connections.repo.
        unimplemented!()
    }

    async fn in_with_email(&self) -> () {
        unimplemented!()
    }
}
