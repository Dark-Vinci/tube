use std::net::SocketAddr;
use axum::Router;
use tokio::{select, signal};
use tracing::trace;

pub(self) async fn graceful_shutdown() {
    let ctr_l = async {
        signal::ctrl_c()
            .await
            .expect("FAILED TO HANDLE CONTROL C")
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("FAILED TO INSTALL SIGNAL HANDLER")
            .recv()
            .await
    };

    #[cfg(not(unix))]
        let terminate = future::pending::<()>();

    select! {
        _ = ctr_l => {},
        _ = terminate => {},
    }

    trace!("SIGNAL RECEIVED🚨: Handling graceful shutdown🛑 server🦾")
}

pub async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}