use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use axum::{Json, response};
use uuid::Uuid;

use crate::helpers::constants::constants::{
    ERROR_MESSAGE, SUCCESS_MESSAGE
};
use crate::model::error_response::AppError;
use crate::model::success_response::SuccessResponse;

#[derive(Serialize)]
pub struct Response<T> {
    pub status_code: u16,
    pub data: Option<SuccessResponse<T>>,
    pub error: Option<AppError>,
    pub message: &'static str,
    pub request_id: String,
}

impl<T> IntoResponse for Response<T> {
    fn into_response(&self) -> response::Response {
        (StatusCode::from_u16(self.status_code), Json(self)).into_response()
    }
}

impl<T> Response<T> {
    pub fn error(
        e: AppError,
        req_id: Uuid,
        status_code: StatusCode,
    ) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: None::<T>,
            error: Some(e),
            message: ERROR_MESSAGE,
            request_id: req_id.into(),
        }
    }

    pub fn success(
        data: SuccessResponse<T>,
        req_id: Uuid,
        status_code: StatusCode,
    ) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: Some(data),
            error: None,
            message: SUCCESS_MESSAGE,
            request_id: req_id.into(),
        }
    }
}
