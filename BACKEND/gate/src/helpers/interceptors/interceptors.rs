use {
    crate::helpers::constants::constants::REQUEST_ID,
    axum::{
        extract::Request,
        http::StatusCode,
        middleware::Next,
        response::{IntoResponse, Response},
    },
    uuid::Uuid,
};

// log request response;
pub async fn log_request_response(
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let response = next.run(request).await;

    tracing::debug!("request response {response:?}");

    Ok(response)
}

// Add request_id to request header(completed)
pub async fn append_request_id_response_formatter(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let uuid = Uuid::new_v4().to_string();

    req.headers_mut()
        .append(REQUEST_ID, (&uuid).parse().unwrap());

    let res = next.run(req).await;

    Ok(res)
}
