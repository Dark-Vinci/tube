use {
    crate::config::config::Config,
    sdk::{
        constants::types::E,
        generated_proto_rs::{
            tube_posts::{posts_client::PostsClient, PingResponse},
            tube_utils::Empty,
        },
    },
    tonic::transport::Channel,
};

#[derive(Debug)]
pub struct Posts(Option<PostsClient<Channel>>);

#[async_trait::async_trait]
pub trait PostBehaviour {
    async fn ping(&mut self) -> Result<PingResponse, String>;
}

impl Posts {
    pub fn empty () -> Self {
        Self(None)
    }

    pub async fn new(c: &Config) -> Result<Self, E> {
        let url: Result<tonic::transport::Endpoint, _> =
            c.reaction_url.clone().try_into();

        if url.is_err() {
            return Ok(Self(None));
        }

        let a = PostsClient::connect(url.unwrap()).await;

        if a.is_err() {
            return Ok(Self(None));
        }

        Ok(Self(Some(a.unwrap())))
    }
}

#[async_trait::async_trait]
impl PostBehaviour for Posts {
    async fn ping(&mut self) -> Result<PingResponse, String> {
        if self.0.is_none() {
            return Err("wetin".into());
        }

        let req = self.0.as_mut().unwrap().ping(Empty {}).await;

        if let Err(e) = req {
            return Err(e.to_string());
        }

        let res = req.unwrap().into_inner();

        Ok(res)
    }
}
