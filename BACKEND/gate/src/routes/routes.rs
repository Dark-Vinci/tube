use axum::Router;

use crate::controllers::fallback::fallback;
use crate::routes::account::Account;
use crate::routes::posts::Post;
use crate::routes::reactions::Reactions;
use crate::routes::timeline::Timeline;

// 3) How we'll handle Query, Body, Param extraction and validation +++
// 4) How to handle validation +++
// 5) How to target a specific ROUTE for guard +++

// 1) We need an interceptor for getting response/error and formatting them into response(not needed);

// 2) interceptor for injecting requestId to every request(headers) ++++
// 6) Figure out how logging should work
// 7) Print request/response +++;
// 8) Define an error struct +++;

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
            .fallback(fallback);
            // .layer(from_fn(log_request_response))
            // .layer(from_fn(append_request_id_response_formatter));

        application_router
    }
}
