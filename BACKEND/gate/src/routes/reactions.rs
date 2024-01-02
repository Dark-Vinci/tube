use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use crate::helpers::interceptors::interceptors::extract_and_validate_body;

#[derive(Debug, validator::Validate, Deserialize)]
struct QueryParams {
    #[validate(rename= "firstName")]
    name: String,
}

pub struct Reactions;

impl Reactions {
    pub fn routes() -> Router {
        let router = Router::new()
            .route("/like-toggle", get(|| async {}).layer(extract_and_validate_body::<QueryParams>))
            .route("/comment", get(|| async {}))
            .route("/remove-comment", get(|| async {}))
            .route("/unlike-toggle", get(|| async {}));

        router
    }
}