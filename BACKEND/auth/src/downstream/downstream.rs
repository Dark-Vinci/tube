use {
    super::{posts::posts::Posts, reactions::reactions::Reaction},
    crate::config::config::Config,
    tokio::join,
};

#[derive(Debug)]
pub struct DownStream {
    pub reactions: Reaction,
    pub posts: Posts,
}

impl DownStream {
    pub async fn new(config: &Config) -> Result<Self, String> {
        let r = Reaction::new(&config);
        let p = Posts::new(&config);

        let (r, p) = join!(r, p);

        if let Err(e) = r {
            return Err(e.to_string());
        }

        if let Err(e) = p {
            return Err(e.to_string());
        }

        Ok(Self {
            reactions: r.unwrap(),
            posts: p.unwrap(),
        })
    }
}
