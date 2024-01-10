use tonic::async_trait;

use super::{traits::Report, application::App};

#[async_trait]
impl Report for App {
    async fn report_user(&self) -> () {
        todo!()
    }

    async fn report_channel(&self) -> () {
        todo!()
    }

    async fn report_short(&self) -> () {
        todo!()
    }

    async fn report_channel_video(&self) -> () {
        todo!()
    }

    async fn report_short_video(&self) -> () {
        todo!()
    }

    async fn report_comment(&self) -> () {
        todo!()
    }
}
