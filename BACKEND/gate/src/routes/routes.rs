use axum::http::{StatusCode, Uri};
use axum::{response::Json};
use axum::{Router};
use axum::response::{IntoResponse};
use serde::Serialize;
// use axum::middleware::from_fn;
use tracing::debug;

// use crate::helpers::interceptors::interceptors::{append_request_id_response_formatter, log_request_response};
use crate::model::error_response::AppError;
use crate::model::response::AppResponse;
use crate::routes::account::Account;
use crate::routes::posts::Post;
use crate::routes::reactions::Reactions;
use crate::routes::timeline::Timeline;

// pub struct Data {}

#[tracing::instrument(name="fall")]
async fn fallback(uri: Uri) -> impl IntoResponse {
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

// 1) We need an interceptor for getting response/error and formatting them into response;
// 2) interceptor for injecting requestId to every request(headers) ++++
// 3) How we'll handle Query, Body, Param extraction and validation +++
// 4) How to handle validation +++
// 5) How to target a specific ROUTE for guard +++
// 6) Figure out how logging should work
// 7) Print request/response +++;
// 8) Define an error struct +++;

#[derive(Serialize)]
struct Data {}


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
            .fallback(fallback);
            // .layer(from_fn(log_request_response))
            // .layer(from_fn(append_request_id_response_formatter));

        application_router
    }
}
