use {
    crate::config::config::Config,
    sdk::{
        constants::types::E,
        generated_proto_rs::tube_reactions::reactions_client::ReactionsClient,
    },
    tonic::transport::Channel,
};

#[derive(Debug)]
pub struct Reaction(Option<ReactionsClient<Channel>>);

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

// impl Reaction {
//     pub async fn ping(&mut self) -> Result<PingResponse, String> {
//         let a = Empty {};
//         let req = self.0.ping(a).await;
//
//         if let Err(e) = req {
//             return Err(e.to_string());
//         }
//
//         let req = req.unwrap().into_inner();
//
//         Ok(req)
//     }
// }
