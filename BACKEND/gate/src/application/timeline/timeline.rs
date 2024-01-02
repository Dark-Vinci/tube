use crate::application::app::App;
use crate::application::traits::TimeLine;

impl TimeLine for App {
    async fn get_timeline(&self) -> Vec<String> {
        todo!()
    }

    async fn get_short_timeline(&self) -> Vec<String> {
        todo!()
    }
}
