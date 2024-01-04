use axum::Json;
use axum::response::IntoResponse;
use http::{StatusCode, Uri};
use tracing::debug;

use crate::model::error_response::AppError;
use crate::model::response::{AppResponse, Data};

#[tracing::instrument(name="fallback-handler", ret)]
pub async fn fallback(uri: Uri) -> impl IntoResponse {
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
        "345c69e7-6fb9-49a3-8b36-afa99ee13557".to_string(),
        StatusCode::NOT_FOUND
    );

    return (StatusCode::NOT_FOUND, Json(app)).into_response();
}
