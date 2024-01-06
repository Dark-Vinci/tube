use tokio::join;
use tonic::async_trait;

use crate::config::config::Config;
use crate::downstream::posts::posts::Posts;
use crate::downstream::reactions::reactions::Reaction;

pub struct DownStream {
    pub reactions: Reaction,
    pub posts: Posts,
}

#[async_trait]
impl DownStream {
    pub async fn new (config: &Config) -> Result<Self, String> {
        let r = Reaction::new(&config);
        let p = Posts::new(&config);

        let (r, p) = join!(r, p);

        if let Err(e) = r {
            Err(e.to_string())
        }

        if let Err(e) = p {
            Err(e.to_string())
        }

        Ok(Self {
            reactions: r.unwrap(),
            posts: p.unwrap(),
        })
    }
}
