use axum::Router;
use axum::routing::get;

pub struct Reactions;

impl Reactions {
    pub fn routes() -> Router {
        let router = Router::new()
            .route("/like-toggle", get(|| async {}))
            .route("/comment", get(|| async {}))
            .route("/remove-comment", get(|| async {}))
            .route("/unlike-toggle", get(|| async {}));

        router
    }
}