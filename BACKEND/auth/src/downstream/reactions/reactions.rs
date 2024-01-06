use sdk::E;
use sdk::generated_proto_rs::tube_reactions::PingResponse;
use sdk::generated_proto_rs::tube_reactions::reactions_client::ReactionsClient;
use tonic::async_trait;
use tonic::transport::Channel;
use sdk::generated_proto_rs::tube_utils::Empty;
use crate::config::config::Config;

#[derive(Default, Debug)]
pub struct Reaction(ReactionsClient<Channel>);

#[async_trait]
impl Reaction {
    pub async fn new(c: &Config) -> Result<Self, E> {
        let a = ReactionsClient::connect(&c.reaction_url).await?;

        Ok(Self(a))
    }
}

#[async_trait]
impl Reaction {
    pub async fn ping(&mut self) -> Result<PingResponse, String> {
        let a = Empty{};
        let req = self.0.ping(a).await;

        if let Err(e) = req {
            return Err(e.to_string());
        }

        let req= req.unwrap().into_inner();

        Ok(req)
    }
}