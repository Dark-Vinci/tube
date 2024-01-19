use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/short", get(|| async {}))
        .route("/normal", get(|| async {}))
}
