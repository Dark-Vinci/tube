use std::net::SocketAddr;

use tonic::transport::Server;
use tracing::debug;

use sdk::E;
use sdk::constants::helper::{LOCAL_HOST, AUTH_NAME, APP_NAME};
use sdk::helpers::shutdown::graceful_shutdown;
use sdk::generated_proto_rs::tube_auth::auth_service_server::AuthServiceServer;

use auth::application::application::App;
use auth::config::config::Config;
use auth::connections::db::DBConnection;
use auth::connections::redis::Redis;
use auth::controller::controller::Auth;
use auth::downstream::downstream::DownStream;
use auth::repository::repository::Repo;

#[tokio::main]
async fn main() -> Result<(), E> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_current_span(false)
        .init();

    // load the config
    let config = Config::new();

    let addr: SocketAddr = format!("{0}:{1}", LOCAL_HOST, &config.app_port).parse()?;

    // connect to necessary network services
    let db = DBConnection::open(&config).await?;
    let redis = Redis::connect(&config).await?;

    // using DB connection, bootstrap repository
    let repo = Repo::new(db);

    // migration[]clone db and use it as connection object;

    // bootstrap the downstream
    let downstream = DownStream::new(&config).await?;

    // bootstrap application
    let app = App::new(config, downstream, repo, redis);

    // bootstrap service controller
    let auth_server = Auth::new(app);

    debug!("🚀{0} for {1} is listening on address {2} 🚀", AUTH_NAME, APP_NAME, addr);

    // start service and listen to shutdown hooks;
    Server::builder()
        .add_service(AuthServiceServer::new(auth_server))
        .serve_with_shutdown(addr, graceful_shutdown())
        .await?;

    Ok(())
}
