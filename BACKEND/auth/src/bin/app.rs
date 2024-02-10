use sdk::constants::types::E;


#[tokio::main]
async fn main() -> Result<(), E> {
    auth::startup().await
}
