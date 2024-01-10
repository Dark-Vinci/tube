
use tokio::{select, signal};

pub async fn graceful_shutdown() {
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

    println!("SIGNAL RECEIVED🚨: Handling graceful shutdown🛑 server🦾")
}
