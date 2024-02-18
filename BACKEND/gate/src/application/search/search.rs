use axum::async_trait;
use uuid::Uuid;

use crate::application::app::App;
use crate::application::traits::Search;

#[allow(unused_variables)]
#[async_trait]
impl Search for App {
    async fn create(&self) {
        todo!()
    }

    async fn get_recent(&self) -> Vec<String> {
        todo!()
    }

    async fn get_all(&self) -> Vec<String> {
        todo!()
    }

    async fn delete_all(&self) -> bool {
        todo!()
    }

    async fn delete_one(&self, id: Uuid) -> bool {
        todo!()
    }

    async fn recover(&self) -> bool {
        todo!()
    }
}
