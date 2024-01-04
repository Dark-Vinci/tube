use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SuccessResponse<T: Serialize> {
    pub data: T,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(
        status_code: StatusCode,
        message: String,
        data: T
    ) -> Self {
        Self {
            data,
            message,
            status_code,
        }
    }
}
