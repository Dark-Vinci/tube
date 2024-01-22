use tracing::{info, trace};

use gate::application::app::App;
use gate::config::config::Config;
use gate::controllers::controllers::Controllers;
use gate::downstream::downstream::DownStream;
use gate::helpers::util::graceful::serve;
use gate::routes::routes::AppRouter;

#[allow(dead_code)]


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_current_span(false)
        .init();

    trace!("About to startup the server");

    // app configurations
    let config = Config::new();
    let port = config.port.clone();

    // downstream to other service
    let downstream = DownStream::new(&config);

    // application service
    let app = App::new(config, downstream);

    // controller(route handlers)
    let controllers = Controllers::new(app);

    // routers
    let router = AppRouter::routes(controllers);

    // let port: u16 = 8080;

    info!("listening on 👂🏿👂🏿👂🏿 port {}", port);

    serve(router, port).await;
}
