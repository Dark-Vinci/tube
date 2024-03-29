use {super::application::App, crate::application::traits::Others, uuid::Uuid};

impl Others for App {
    fn ping(&self, request_id: Uuid) -> String {
        return format!("Others respond to {}", request_id.to_string());
    }
}
