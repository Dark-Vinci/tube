use {
    crate::config::config::Config,
    sdk::{
        constants::types::E,
        generated_proto_rs::tube_reactions::reactions_client::ReactionsClient,
        generated_proto_rs::tube_reactions::PingResponse,
        generated_proto_rs::tube_utils::Empty,
    },
    tonic::transport::Channel,
};

#[derive(Debug)]
pub struct Reaction(ReactionsClient<Channel>);

impl Reaction {
    pub async fn new(c: &Config) -> Result<Self, E> {
        let url: Result<tonic::transport::Endpoint, _> =
            c.reaction_url.clone().try_into();
        let a = ReactionsClient::connect(url.unwrap()).await?;

        Ok(Self(a))
    }
}

impl Reaction {
    pub async fn ping(&mut self) -> Result<PingResponse, String> {
        let a = Empty {};
        let req = self.0.ping(a).await;

        if let Err(e) = req {
            return Err(e.to_string());
        }

        let req = req.unwrap().into_inner();

        Ok(req)
    }
}
