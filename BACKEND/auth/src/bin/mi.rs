use axum::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::routing::get;
use tokio::task;

async fn handle_request() -> axum::response::Html<&'static str> {
    axum::response::Html("<h1>Hello, Axum!</h1>")
}

#[tokio::main]
async fn main() {
    // Get the number of available CPUs
    let num_cpus = num_cpus::get();

    // Create a vector to hold the join handles of worker tasks
    let mut handles = Vec::new();

    // Create an Arc containing the Axum app
    let app = Arc::new(
        route("/", get(handle_request))
    );

    // Spawn a worker task for each CPU
    for _ in 0..num_cpus {
        let app = Arc::clone(&app);
        let handle = task::spawn(async move {
            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            let server = Server::bind(&addr)
                .serve(app)
                .await
                .unwrap();
            server.await.unwrap();
        });
        handles.push(handle);
    }

    // Wait for all worker tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}
