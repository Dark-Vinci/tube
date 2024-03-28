use {
    crate::application::{app::App, traits::TimeLine},
    axum::async_trait,
};

#[async_trait]
impl TimeLine for App {
    async fn get_timeline(&self) -> Vec<String> {
        todo!()
    }

    async fn get_short_timeline(&self) -> Vec<String> {
        todo!()
    }
}
