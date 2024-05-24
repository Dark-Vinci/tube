use {
    super::{application::App, traits::Ban},
    tonic::async_trait,
};

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

#[cfg(test)]
mod test {
    use {super::App, crate::application::traits::Others, uuid::Uuid};
    use crate::application::traits::Ban;

    async fn get_app() -> App {
        return App::new(Default::default()).await.unwrap();
    }

    // create stub service for Redis, DB, RabbitMQ, use it to create the App instance
    #[tokio::test]
    async fn ban_user_test() {
        let a = get_app().await;
        let res = a.ban_user().await;

        assert_eq!(res, ());
    }

    #[tokio::test]
    async fn ban_channel_test() {
        let a = get_app().await;
        let res = a.unban_channel().await;

        assert_eq!(res, ());
    }

    #[tokio::test]
    async fn unban_user_test() {
        let a = get_app().await;
        let res = a.unban_user().await;

        assert_eq!(res, ());
    }

    #[tokio::test]
    async fn unban_channel_test() {
        let a = get_app().await;
        let res = a.unban_channel().await;

        assert_eq!(res, ());
    }

    #[tokio::test]
    async fn ban_short_test() {
        let a = get_app().await;
        let res = a.unban_short().await;

        assert_eq!(res, ());
    }

    #[tokio::test]
    async fn unban_short_test() {
        let a = get_app().await;
        let res = a.unban_short().await;

        assert_eq!(res, ());
    }
}
