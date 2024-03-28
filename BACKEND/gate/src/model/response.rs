use {
    crate::{
        helpers::constants::constants::{ERROR_MESSAGE, SUCCESS_MESSAGE},
        model::{error_response::AppError, success_response::SuccessResponse},
    },
    axum::http::StatusCode,
    serde::Serialize,
    uuid::Uuid,
};

#[derive(Serialize, Clone)]
pub struct Data {}

#[derive(Serialize, Debug, Clone)]
pub struct AppResponse<T: Serialize> {
    pub status_code: u16,
    pub data: Option<SuccessResponse<T>>,
    pub error: Option<AppError>,
    pub message: &'static str,
    pub request_id: String,
}

impl<T: Serialize> AppResponse<T> {
    pub fn error(e: AppError, request_id: String, status_code: StatusCode) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: None,
            error: Some(e),
            message: ERROR_MESSAGE,
            request_id,
        }
    }

    pub fn success(
        data: SuccessResponse<T>,
        request_id: Uuid,
        status_code: StatusCode,
    ) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: Some(data),
            error: None,
            message: SUCCESS_MESSAGE,
            request_id: request_id.into(),
        }
    }
}
