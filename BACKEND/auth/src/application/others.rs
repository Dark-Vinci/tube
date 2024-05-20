use {super::application::App, crate::application::traits::Others, uuid::Uuid};

impl Others for App {
    fn ping(&self, request_id: Uuid) -> String {
        // let mut b = "This string will be read".as_bytes();
        format!("Others respond to {}", request_id)
    }
}
// include_str!(for HTML template)
