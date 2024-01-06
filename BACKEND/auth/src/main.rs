use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::debug;
use sdk::generated_proto_rs::tube_auth::auth_service_server::AuthServiceServer;

use auth::controller::controller::Auth;

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
        .serve(addr)
        .await?;

    Ok(())
}
