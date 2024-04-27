use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    #[serde(rename="userId")]
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp: Vec<Post> = reqwest::get("https://jsonplaceholder.typicode.com/posts")
        .await?
        .json()
        .await?;

    use std::time::{Instant, Duration};

    let now = Instant::now();
    let max_seconds = u64::MAX / 1_000_000_000;
    let duration = Duration::new(max_seconds, 0);
    println!("{:?}", now + duration);

    // let abc = &resp.iter().filter(|&a| a.id < 10).collect::<Vec<Post>>();
    // println!("{abc:#?}");

    let hun = resp.iter().find(|&a| a.id == 100);

    if hun.is_none() {
        println!("NOT FOUND")
    }

    let hun = &hun.unwrap().body;

    println!("{hun:#?}");
    Ok(())
}
