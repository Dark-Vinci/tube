use axum::Router;
use axum::routing::get;

use axum::middleware::{ from_fn};
use serde::Deserialize;
use crate::helpers::interceptors::interceptors::extract_and_validate_query;

#[derive(Debug, validator::Validate, Deserialize)]
struct QueryParams {
    #[validate(rename= "firstName")]
    name: String,
}

pub struct Post;

impl Post {
    pub fn routes() -> Router{
        let router = Router::new()
            .route("/create", get(|| async {}).layer(from_fn(extract_and_validate_query)))
            .route("/delete", get(|| async {}))
            .route("/report", get(|| async {}))
            .route("/update", get(|| async {}));

        router
    }
}