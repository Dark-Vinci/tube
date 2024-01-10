use tonic::async_trait;

use super::{application::App, traits::Subscribe};

#[async_trait]
impl Subscribe for App {
    async fn toggle(&self) -> () {
        todo!()
    }

    async fn get_subscribers(&self) -> () {
        todo!()
    }
}
