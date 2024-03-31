use {
    super::{application::App, traits::Report},
    tonic::async_trait,
};

#[async_trait]
impl Report for App {
    async fn report_user(&self) -> () {
        // self.ban_repo.delete_by_id(request_id, id)
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
