use {
    crate::config::config::Config,
    async_trait::async_trait,
    sdk::{
        constants::types::E,
        generated_proto_rs::{
            tube_reactions::{reactions_client::ReactionsClient, PingResponse},
            tube_utils::Empty,
        },
    },
    tonic::transport::Channel,
};

#[derive(Debug)]
pub struct Reaction(Option<ReactionsClient<Channel>>);

#[async_trait]
pub trait ReactionBehaviour {
    async fn ping(&mut self) -> Result<PingResponse, String>;
}

impl Reaction {
    pub async fn new(c: &Config) -> Result<Self, E> {
        let url: Result<tonic::transport::Endpoint, _> =
            c.reaction_url.clone().try_into();

        if url.is_err() {
            return Ok(Self(None));
        }

        let a = ReactionsClient::connect(url.unwrap()).await;

        if a.is_err() {
            return Ok(Self(None));
        }

        Ok(Self(Some(a.unwrap())))
    }
}

#[async_trait]
impl ReactionBehaviour for Reaction {
    async fn ping(&mut self) -> Result<PingResponse, String> {
        if self.0.is_none() {
            return Err("wetin".into());
        }

        let req = self.0.as_mut().unwrap().ping(Empty {}).await;

        if let Err(e) = req {
            return Err(e.to_string());
        }

        let req = req.unwrap().into_inner();

        Ok(req)
    }
}
