use {
    crate::{
        helpers::middleware::request_id_extractor::{GetRequestID, RequestId},
        model::{
            error_response::AppError,
            response::{AppResponse, Data},
            success_response::SuccessResponse,
        },
    },
    axum::{response::IntoResponse, Json},
    http::{StatusCode, Uri},
    tracing::debug,
};

#[tracing::instrument(name = "fallback-handler", ret)]
pub async fn fallback(uri: Uri, RequestId(id): RequestId) -> impl IntoResponse {
    debug!("Got a request on fallback");

    let error = AppError::new(
        StatusCode::NOT_FOUND,
        format!("no such resource found in the uri {uri}").into(),
        "html".to_string(),
        "fallback".into(),
        false,
    );

    if uri.to_string() == "fallback" {
        bail!(AppError);
    }

    let app: AppResponse<Data> =
        AppResponse::error(error, id.to_string(), StatusCode::NOT_FOUND);

    return (StatusCode::NOT_FOUND, Json(app)).into_response();
}

#[tracing::instrument(name = "pinging", ret)]
pub async fn ping(GetRequestID(id): GetRequestID) -> impl IntoResponse {
    debug!("Got a Pinging healthcheck request");

    let error = SuccessResponse::new(
        StatusCode::NOT_FOUND,
        format!("hello from pinngy with UUID: {}", id.to_string()).into(),
        Data {},
    );

    let app: AppResponse<Data> = AppResponse::success(error, id, StatusCode::OK);

    return (StatusCode::NOT_FOUND, Json(app)).into_response();
}
