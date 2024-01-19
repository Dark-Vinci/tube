use axum::middleware::from_extractor;
use axum::routing::get;
use axum::Router;
use http::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::fallback::{fallback, ping};
use crate::helpers::middleware::request_id_extractor::RequestID;

use super::{account, posts, reactions, timeline};

pub struct AppRouter;

impl AppRouter {
    pub fn routes() -> Router {
        let cors = CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
            ])
            .allow_origin(Any)
            .expose_headers(Any);

        let timeline_routes = timeline::routes();
        let reaction_routes = reactions::routes();
        let account_routes = account::routes();
        let post_routes = posts::routes();

        Router::new()
            .nest("/posts", post_routes)
            .nest("/account", account_routes)
            .nest("/reactions", reaction_routes)
            .nest("/timeline", timeline_routes)
            .route("/health", get(ping))
            .fallback(fallback)
            .route_layer(from_extractor::<RequestID>())
            .layer(cors)
        // .layer(from_fn(log_request_response))
        // .layer(from_fn(append_request_id_response_formatter));
    }
}
