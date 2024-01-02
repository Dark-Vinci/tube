use axum::Router;
use axum::routing::get;

pub struct Account;

impl Account {
    pub fn routes() -> Router {
        let router = Router::new()
            .route("/sign-in-with-google", get(|| async {}))
            .route("/sign-in-with-email", get(|| async {}))
            .route("/signup-with-google", get(|| async {}))
            .route("/signup-with-email", get(|| async {}))
            .route("/me", get(|| async {}))
            .route("/report", get(|| async {}));

        router
    }
}