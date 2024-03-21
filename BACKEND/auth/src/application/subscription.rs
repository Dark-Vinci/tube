use {
    super::{application::App, traits::Subscribe},
    tonic::async_trait,
};

#[async_trait]
impl Subscribe for App {
    async fn toggle(&self) -> () {
        todo!()
    }

    async fn get_subscribers(&self) -> () {
        todo!()
    }
}
