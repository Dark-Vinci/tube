use axum::response::IntoResponse;
use axum::Json;
use http::{StatusCode, Uri};
use tracing::debug;

use crate::helpers::middleware::request_id_extractor::RequestId;
use crate::model::error_response::AppError;
use crate::model::response::{AppResponse, Data};
use crate::model::success_response::SuccessResponse;

#[tracing::instrument(name = "fallback-handler", ret)]
pub async fn fallback(
    uri: Uri,
    RequestId(id): RequestId,
) -> impl IntoResponse {
    debug!("Got a request on fallback");

    let error = AppError::new(
        StatusCode::NOT_FOUND,
        format!("no such resource found in the uri {uri}").into(),
        "html".to_string(),
        "fallback".into(),
        false,
    );

    let app: AppResponse<Data> = AppResponse::error(
        error,
        id.to_string(),
        StatusCode::NOT_FOUND,
    );

    return (StatusCode::NOT_FOUND, Json(app)).into_response();
}

pub async fn ping(RequestId(id): RequestId) -> impl IntoResponse {
    let error = SuccessResponse::new(
        StatusCode::NOT_FOUND,
        format!("hello from pinngy with UUID: {}", id.to_string())
            .into(),
        Data {},
    );

    let app: AppResponse<Data> =
        AppResponse::success(error, id, StatusCode::OK);

    return (StatusCode::NOT_FOUND, Json(app)).into_response();
}
