use {
    super::{account, posts, reactions, timeline},
    crate::{
        controllers::{
            controllers::Controllers,
            fallback::{fallback, ping},
        },
        helpers::middleware::request_id_extractor::RequestID,
    },
    axum::{middleware::from_extractor, routing::get, Router},
    http::Method,
    tower_http::cors::{Any, CorsLayer},
};

pub struct AppRouter;

impl AppRouter {
    pub fn routes(_c: Controllers) -> Router {
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
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
