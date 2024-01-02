use std::ops::Add;

use axum::{Extension};
use axum::extract::{FromRequest, Query, Request, Json};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::helpers::constants::constants::REQUEST_ID;

// log request response;
pub async fn log_request_response(
    request: Request,
    next: Next
) -> Result<impl IntoResponse, Response> {
    let request_clone = request.clone().body();

    let response = next.run(request).await;

    tracing::debug!("request {request_clone:?}, response {response:?}");

    Ok(response)
}

// Add request_id to request header(completed)
pub async fn append_request_id_response_formatter<T>(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let uuid = Uuid::new_v4().to_string();

    req.headers_mut().append(REQUEST_ID, (&uuid).parse().unwrap());

    let res = next.run(req).await;

    Ok(res)
}

// Validate request param(completed)
pub async fn extract_and_validate_param<T: Validate>(
    request: Request,
    next: Next
) -> Result<Response, String> {
    // let Request(_res) = request;
    let param_result =  Extension::<T>::from_request(request.clone(), &()).await;

    if let Err(e) = param_result {
        return Err(String::from(e));
    }

    let Extension(param) = param_result.unwrap();

    let new_p = param.validate();

    if let Err(e) = new_p {
        return Err(collect_error(e));
    }

    let res = next.run(request).await;

    return Ok(res);
}

// format validation error
fn collect_error(param: ValidationErrors) -> String {
    let mut message = String::new();

    for (field, error_message) in param.field_errors() {
        let mut error_messages = String::new();

        for er in error_message {
            let ms = &er.to_string();
            error_messages.add(ms);
        }

        message = format!("{message}, Field {field}: Message: {error_messages}").to_string();
    }

    return message;
}

// Validate request query(completed)
pub async fn extract_and_validate_query<T: Validate>(
    request: Request,
    next: Next
) -> Result<Response, String> {
    let query = Query::<T>::from_request(request.clone(), &()).await;

    if let Err(e) = query {
        return Err(e.to_string());
    }

    let Query(query) = query.unwrap();

    let query = query.validate();

    if let Err(e) = query {
        return Err(collect_error(e));
    }

    let res = next.run(request).await;

    return Ok(res);
}

// Validate request query(completed)
pub async fn extract_and_validate_body<T: Validate>(
    request: Request,
    next: Next
) -> Result<Response, String> {
    let body: Result<Json<T>, _> = FromRequest::from_request(request.clone(), &()).await;

    if let Err(e) = body {
        return Err(e.to_string());
    }

    let Json(new_body) = body.unwrap();

    let body_validation = new_body.validate();

    if let Err(e) = body_validation {
        return Err(collect_error(e));
    }

    let res = next.run(request).await;

    return Ok(res);
}
