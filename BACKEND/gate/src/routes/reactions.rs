use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/like-toggle", get(|| async {}))
        .route("/comment", get(|| async {}))
        .route("/remove-comment", get(|| async {}))
        .route("/unlike-toggle", get(|| async {}))
}
