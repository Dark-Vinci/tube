use axum::middleware::from_extractor;
use axum::Router;

use crate::controllers::fallback::fallback;
use crate::helpers::middleware::request_id_extractor::{
    RequestId, 
    RequestID
};
use super::account::Account;
use super::posts::Post;
use super::reactions::Reactions;
use super::timeline::Timeline;

pub struct AppRouter;

impl AppRouter {
    pub fn routes() -> Router {
        let timeline_routes = Timeline::routes();
        let reaction_routes = Reactions::routes();
        let account_routes = Account::routes();
        let post_routes = Post::routes();

        let application_router = Router::new()
            .nest("/posts", post_routes)
            .nest("/account", account_routes)
            .nest("/reactions", reaction_routes)
            .nest("/timeline", timeline_routes)
            .fallback(fallback)
            .route_layer(from_extractor::<RequestID>());
            // .layer(from_fn(log_request_response))
            // .layer(from_fn(append_request_id_response_formatter));

        application_router
    }
}
