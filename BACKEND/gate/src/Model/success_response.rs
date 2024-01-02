use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
struct SuccessResponse<T: Serialize> {
    data: T,
    status_code: StatusCode,
    message: String,
}

impl<T> SuccessResponse<T> {
    pub fn new(
        status_code: StatusCode,
        message: String,
        data: T
    ) -> Self<T> {
        Self {
            data,
            message,
            status_code,
        }
    }
}

