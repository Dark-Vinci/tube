use axum::Router;
use axum::routing::get;

pub struct Post;

impl Post {
    pub fn routes() -> Router{
        let router = Router::new()
            .route("/create", get(|| async { return "what are we talking about" }))
            .route("/delete", get(|| async {}))
            .route("/report", get(|| async {}))
            .route("/update", get(|| async {}));

        router
    }
}