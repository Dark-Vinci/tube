use std::fmt::Debug;
use axum::body::{Body, Bytes};
use axum::Extension;
use axum::extract::{FromRequest, FromRequestParts, Query, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::helpers::constants::constants::REQUEST_ID;

// #[derive(Debug, Validate, Deserialize)]
// struct QueryParams {
//     #[validate(rename= "firstName")]
//     name: String,
// }
//
//
// #[derive(Debug, Deserialize, Validate)]
// struct PathParams {
//     id: u32,
// }

// async fn handler() -> Result<(), AppError:IntoResponse>

async fn print_request_body(request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

// log request response;
pub async fn log_request_response(
    request: Request,
    next: Next
) -> Result<impl IntoResponse, Response> {
    let request = buffer_request_body(request).await?;

    let response = next.run(request).await;

    tracing::debug!("request {request}, response {response}");

    Ok(response)
}

// Add request_id to request header(completed)
pub async fn append_request_id_response_formatter(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let uuid = Uuid::new_v4();

    req.headers_mut().append(REQUEST_ID, uuid.into());

    let res = next.run(req).await;



    Ok(res)
}

// Validate request param(completed)
pub async fn extract_and_validate_param<T: Validate>(
    request: Request,
    next: Next
) -> Result<Response, String> {
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
        write!(&mut message, "Field {field}: Message: {error_message}").unwrap();
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
    let query = Query::<T>::from_request(request.clone(), &()).await;

    let (parts, body) = request.clone().into_parts();



    // body.fmt()

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

// the trick is to take the request apart, buffer the body, do what you need to do, then put
// the request back together
async fn buffer_request_body(request: Request) -> Result<Request, Response> {
    // request.
    let (parts, body) = request.clone().into_parts();

    let Query(query) = Query::<QueryParams>::from_request_parts(&mut parts.clone(), &()).await.unwrap();
    let Extension(param) =  Extension::<PathParams>::from_request(request, &()).await.unwrap();

    // this wont work if the body is an long running stream
    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    do_thing_with_request_body(bytes.clone());

    Ok(Request::from_parts(parts, Body::from(bytes)))
}

fn do_thing_with_request_body(bytes: Bytes) {
    tracing::debug!(body = ?bytes);
}