#[derive(Debug)]
pub struct DownStream {
    pub util: String,
    pub post: String,
    pub account: String,
    pub reactions: String,
}

impl DownStream {
    pub fn new () -> Self {
        Self {
            util: "".to_string(),
            post: "".to_string(),
            account: "".to_string(),
            reactions: "".to_string(),
        }
    }
}
