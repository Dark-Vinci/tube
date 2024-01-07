use tracing::{info, trace};

use gate::helpers::util::graceful::serve;
use gate::routes::routes::AppRouter;

#[allow(dead_code)]

// config
// downstream
// application
// controller
// route

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_current_span(false)
        .init();

    trace!("About to startup the server");

    let app = AppRouter::routes();

    let port: u16 = 8080;

    info!("listening on 👂🏿👂🏿👂🏿 port {}", port);

    serve(app, port).await;
}
