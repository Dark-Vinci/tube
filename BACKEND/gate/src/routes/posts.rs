use std::fmt::{Debug, Display, Formatter, write};
use axum::Router;
use axum::debug_handler;
use axum::routing::get;
use serde::Deserialize;

use validator::Validate;
use crate::helpers::middleware::body_extractor::BodyValidator;
use crate::helpers::middleware::param_extractor::ParamValidator;
use crate::helpers::middleware::query_extractor::QueryValidator;

pub struct Post;

#[derive(Validate, Debug, Clone, Deserialize)]
pub struct Meme {
    #[validate(length(max = 3))]
    id: String
}

impl Display for Meme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[debug_handler]
async fn dele(vm: BodyValidator<Meme>) -> String {
    // tracing::debug!(?body, "handler received body");
    let BodyValidator(v) = vm;
    println!("deleppp: {v}");
    return "meme".to_string();
    // return "meme".to_string()
}

impl Post {
    pub fn routes() -> Router {
        let router = Router::new()
            .route("/create", get(|| async { return "what are we talking about" }))
            .route("/delete", get(dele))
            .route("/report", get(|| async {}))
            .route("/update", get(|| async {}));

        router
    }
}