use {
    crate::application::{application::App, traits::SignUp},
    tonic::async_trait,
};

#[async_trait]
impl SignUp for App {
    async fn up_with_google(&self) -> () {
        unimplemented!()
    }

    async fn up_with_email(&self) -> () {
        unimplemented!()
    }
}
