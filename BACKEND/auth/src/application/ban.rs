use tonic::async_trait;

use super::{application::App, traits::Ban};

#[async_trait]
impl Ban for App {
    async fn ban_user(&self) -> () {
        todo!()
    }

    async fn ban_channel(&self) -> () {
        todo!()
    }

    async fn ban_short(&self) -> () {
        todo!()
    }

    async fn unban_user(&self) -> () {
        todo!()
    }

    async fn unban_channel(&self) -> () {
        todo!()
    }

    async fn unban_short(&self) -> () {
        todo!()
    }
}
