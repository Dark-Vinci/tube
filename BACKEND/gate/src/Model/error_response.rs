use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub status_code: StatusCode,
    pub public_message: String,
    pub private_message: String,
    pub method_name: String,
    pub fatal: bool,
}

impl Default for AppError {
    fn default() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            fatal: false,
            private_message: "".to_string(),
            public_message: "".to_string(),
            method_name: "".to_string(),
        }
    }
}

impl AppError {
    pub fn new(
        status_code: StatusCode,
        public_message: String,
        private_message: String,
        method_name: String,
        fatal: bool,
    ) -> Self {
        Self {
            fatal,
            private_message,
            public_message,
            method_name,
            status_code,
        }
    }
}