use std::iter::from_fn;
use axum::http::{StatusCode, Uri};
use axum::{response::Json, Router};
use axum::response::{IntoResponse, IntoResponseParts, ResponseParts};
use serde::Serialize;

use axum::middleware::{self, Next};
use crate::helpers::interceptors::interceptors::{append_request_id, log_request_response};

use crate::routes::account::Account;
use crate::routes::posts::Post;
use crate::routes::reactions::Reactions;
use crate::routes::timeline::Timeline;

async fn fallback(uri: Uri) -> impl IntoResponse {
    let error = Err {
        public_message: format!("no such resource found in the uri {uri}"),
        file_name: file!().into(),
        fatal: false,
        method_name: "fallback".into(),
        private_message: "hjemel".to_string(),
    };
    let response = Response {
        status_code: StatusCode::NOT_FOUND.as_u16(),
        data: None,
        error: Option::from(error),
        message: "failure",
        request_id: "345c69e7-6fb9-49a3-8b36-afa99ee13557".to_string(),
    };

    return AppResponse::Failure(response);
}

struct Success<T> {
    data: T,
    request_id: String,
    message: &'static str,
}

enum AppResponse {
    Success(Response),
    Failure(Response),
}

// impl IntoResponseParts for AppResponse {
//     type Error = ();
//
//     fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
//         todo!()
//     }
// }

// 1) We need an interceptor for getting response/error and formatting them into response;
// 2) interceptor for injecting requestId to every request(headers) ++++
// 3) How we'll handle Query, Body, Param extraction and validation
// 4) How to handle validation
// 5) How to target a specific ROUTE for guard
// 6) Figure out how logging should work
// 7) Print request/response
// 8) Define an error struct;

impl IntoResponse for AppResponse {
    fn into_response(self) -> axum::response::Response {
        return match self {
            Self::Success(val) => {
                (StatusCode::from_u16(val.status_code), Json::from(val)).into_response()
            }

            Self::Failure(val) => {
                (StatusCode::from_u16(val.status_code), Json::from(val)).into_response()
            }
        }
    }
}

#[derive(Serialize)]
struct Response {
    status_code: u16,
    data: Option<Data>,
    error: Option<Err>,
    message: &'static str,
    request_id: String,
}

#[derive(Serialize)]
struct Data {}

#[derive(Serialize)]
struct Err {
    public_message: String,
    private_message: String,
    method_name: String,
    file_name: String,
    fatal: bool,
}


pub struct AppRouter;

impl AppRouter {
    pub fn routes() -> Router {
        let timeline_routes = Timeline::routes();
        let reaction_routes = Reactions::routes();
        let account_routes = Account::routes();
        let post_routes = Post::routes();

        let application_router = Router::new()
            .nest("/posts", post_routes)
            .nest("/account", account_routes)
            .nest("/reactions", reaction_routes)
            .nest("/timeline", timeline_routes)
            .fallback(fallback)
            .layer(from_fn(log_request_response))
            .layer(from_fn(append_request_id));

        application_router
    }
}
