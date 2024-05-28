use {
    super::application::App, crate::application::traits::Others, std::cell::RefCell,
    uuid::Uuid,
};

impl Others for App {
    fn ping(&self, request_id: Uuid) -> String {
        // let mut b = "This string will be read".as_bytes();
        format!("Others respond to {}", request_id)
    }
}
// include_str!(for HTML template)
#[cfg(test)]
mod test {
    use {super::App, crate::application::traits::Others, uuid::Uuid};

    async fn get_app() -> App {
        return App::new(Default::default()).await.unwrap();
    }

    // create stub service for Redis, DB, RabbitMQ, use it to create the App instance
    #[tokio::test]
    async fn ping_test() {
        let a = get_app().await;
        let id = Uuid::new_v4();
        let res = a.ping(id);

        assert!(res.starts_with("Others respond to"));
    }
}
