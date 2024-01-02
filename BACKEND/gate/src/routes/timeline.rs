use axum::Router;
use axum::routing::get;

pub struct Timeline;

impl Timeline {
    pub fn routes() -> Router{
        let router = Router::new()
            .route("/short", get(|| async {}))
            .route("/normal", get(|| async {}));

        return router;
    }
}