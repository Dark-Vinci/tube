use {
    crate::config::config::Config,
    sdk::{
        constants::types::E, generated_proto_rs::tube_posts::posts_client::PostsClient,
    },
    tonic::transport::Channel,
};

#[derive(Debug)]
pub struct Posts(Option<PostsClient<Channel>>);

impl Posts {
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

// impl Posts {
//     pub async fn ping(&mut self) -> Result<PingResponse, String> {
//         let req = self.0.ping(Empty {}).await;
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
