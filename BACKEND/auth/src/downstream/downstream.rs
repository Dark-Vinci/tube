use {
    super::{
        posts::posts::{PostBehaviour, Posts},
        reactions::reactions::{Reaction, ReactionBehaviour},
    },
    crate::config::config::Config,
    sdk::generated_proto_rs::tube_posts,
    tokio::join,
};

#[derive(Debug)]
pub struct DownStream<A, B>
where
    A: ReactionBehaviour,
    B: PostBehaviour,
{
    pub reactions: Box<A>,
    pub posts: Box<B>,
}

#[async_trait::async_trait]
pub trait DownstreamBehaviour {
    async fn post_ping(&mut self) -> Result<tube_posts::PingResponse, String>;
}

#[async_trait::async_trait]
impl<A: ReactionBehaviour + std::marker::Send, B: PostBehaviour + std::marker::Send> DownstreamBehaviour for DownStream<A, B> {
    async fn post_ping(&mut self) -> Result<tube_posts::PingResponse, String> {
        self.posts.ping().await
    }
}

impl<A: ReactionBehaviour, B: PostBehaviour> DownStream<A, B> {
    pub async fn new(config: &Config) -> Result<Self, String> {
        let r = Reaction::new(config);
        let p = Posts::new(config);

        let (r, p) = join!(r, p);

        if let Err(e) = r {
            return Err(e.to_string());
        }

        if let Err(e) = p {
            return Err(e.to_string());
        }

        let r: Box<A> = Box::new(r.unwrap());

        Ok(Self {
            reactions: Box::new(r.unwrap()),
            posts: p.unwrap(),
        })
    }
}
