use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/sign-in-with-google", get(|| async {}))
        .route("/sign-in-with-email", get(|| async {}))
        .route("/signup-with-google", get(|| async {}))
        .route("/signup-with-email", get(|| async {}))
        .route("/me", get(|| async {}))
        .route("/report", get(|| async {}))
}
