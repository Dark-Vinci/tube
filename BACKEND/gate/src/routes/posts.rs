use std::fmt::{Debug, Display, Formatter};

use axum::debug_handler;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use validator::Validate;

use crate::helpers::middleware::request_id_extractor::GetRequestID;

// pub struct Post;

#[derive(Validate, Debug, Clone, Deserialize)]
pub struct Meme {
    #[validate(length(max = 3))]
    id: String,
}

impl Display for Meme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[debug_handler]
async fn dele(GetRequestID(val): GetRequestID) -> String {
    // tracing::debug!(?body, "handler received body");
    // let BodyValidator(v) = vm;
    println!("deleppp: {}", val.to_string());
    return val.to_string();
    // return "meme".to_string()
}

pub fn routes() -> Router {
    Router::new()
        .route(
            "/create",
            get(|| async {
                return "what are we talking about";
            }),
        )
        .route("/delete", get(dele))
        .route("/report", get(|| async {}))
        .route("/update", get(|| async {}))
}
