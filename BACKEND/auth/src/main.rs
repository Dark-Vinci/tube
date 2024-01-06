use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::debug;
use sdk::generated_proto_rs::tube_auth::auth_service_server::AuthServiceServer;
use sdk::helpers::shutdown::graceful_shutdown;

use auth::controller::controller::Auth;

// config
// connections
// application
// controller

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    let greeter = Auth::default();

    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_current_span(false)
        .init();

    debug!("🚀GreeterServer listening on {} 🚀", addr);

    Server::builder()
        .add_service(AuthServiceServer::new(greeter))
        .serve_with_shutdown(addr, graceful_shutdown())
        .await?;

    Ok(())
}
