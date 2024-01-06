use tonic::{Request, Response, Status, async_trait};
use sdk::generated_proto_rs::tube_utils::Empty;
use sdk::generated_proto_rs::tube_auth::auth_service_server::AuthService;
use sdk::generated_proto_rs::tube_auth::{
    PingResponse,
    SayHelloRequest,
    SayHelloResponse
};

// use crate::application::application::App;
// use crate::application::traits::SignIn;
// use crate::application::traits::SignIn;

#[derive(Default)]
pub struct Auth {
    // pub (crate) app: App,
}

// impl Auth {
//     pub fn new(a: App) -> Self {
//         Self {
//             app: a,
//         }
//     }
// }

#[async_trait]
impl AuthService for Auth {
    async fn ping(&self, _request: Request<Empty>) -> Result<Response<PingResponse>, Status> {
        // let a = &self.app.in_with_email().await;
        let res = PingResponse {
            message: "Pinging".to_string(),
        };

        Ok(Response::new(res))
    }

    async fn say_hello(&self, request: Request<SayHelloRequest>) -> Result<Response<SayHelloResponse>, Status> {
        let SayHelloRequest{ name, request_id } = request.into_inner();

        let res = SayHelloResponse {
            message: format!("Hello {} from say hello method", name).to_string(),
            request_id,
        };

        Ok(Response::new(res))
    }
}