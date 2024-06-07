use {
    super::{
        posts::posts::{PostBehaviour, Posts},
        reactions::reactions::{Reaction, ReactionBehaviour},
    },
    crate::config::config::Config,
    async_trait::async_trait,
    sdk::{
        errors::general::GRPCError,
        generated_proto_rs::{tube_posts, tube_reactions},
    },
    tokio::join,
};

#[derive(Debug)]
pub struct DownStream {
    pub reactions: Reaction,
    pub posts: Posts,
}

#[async_trait]
pub trait DownstreamBehaviour {
    async fn post_ping(&mut self) -> Result<tube_posts::PingResponse, String>;
    async fn reactions_ping(&mut self) -> Result<tube_reactions::PingResponse, String>;
}

#[async_trait]
impl DownstreamBehaviour for DownStream {
    async fn post_ping(&mut self) -> Result<tube_posts::PingResponse, String> {
        self.posts.ping().await
    }

    async fn reactions_ping(&mut self) -> Result<tube_reactions::PingResponse, String> {
        self.reactions.ping().await
    }
}

impl DownStream {
    pub async fn new(
        config: &Config,
    ) -> Result<Box<dyn DownstreamBehaviour + Send + Sync>, GRPCError> {
        let r = Reaction::new(config);
        let p = Posts::new(config);

        let (r, p) = join!(r, p);

        if let Err(e) = r {
            return Err(GRPCError::UnableToConnect(e.to_string()));
        }

        if let Err(e) = p {
            return Err(GRPCError::UnableToConnect(e.to_string()));
        }

        let a = r.unwrap();
        let b = p.unwrap();

        Ok(Box::new(Self {
            reactions: a,
            posts: b,
        }))
    }
}
