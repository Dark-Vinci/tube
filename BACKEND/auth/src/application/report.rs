use {
    super::{application::App, traits::Report},
    sdk::models::db::auth::ban,
    tonic::async_trait,
    uuid::Uuid,
};

#[async_trait]
impl Report for App {
    async fn report_user(&self) -> () {
        // self.ban_repo.create(request_id, b)
        // let a = self
        //     .connections
        //     .bb
        //     .create_ban(
        //         Uuid::new_v4(),
        //         ban::Model {
        //             id: todo!(),
        //             user_id: todo!(),
        //             is_active: todo!(),
        //             created_at: todo!(),
        //             updated_at: todo!(),
        //             deleted_at: todo!(),
        //             expires_at: todo!(),
        //         },
        //     )
        //     .await;
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
