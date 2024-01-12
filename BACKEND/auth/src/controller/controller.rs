use sdk::generated_proto_rs::tube_auth::auth_service_server::AuthService;
use sdk::generated_proto_rs::tube_auth::{
    PingResponse, SayHelloRequest, SayHelloResponse,
};
use sdk::generated_proto_rs::tube_utils::Empty;
use tonic::{async_trait, Request, Response, Status};

use crate::application::application::App;

#[derive(Debug)]
pub struct Auth(App);

impl Auth {
    pub fn new(a: App) -> Self {
        Self(a)
    }
}

#[async_trait]
impl AuthService for Auth {
    async fn ping(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<PingResponse>, Status> {
        let res = PingResponse {
            message: "Pinging".to_string(),
        };

        Ok(Response::new(res))
    }

    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        let SayHelloRequest { name, request_id } =
            request.into_inner();

        let res = SayHelloResponse {
            message: format!("Hello {} from say hell method", name)
                .to_string(),
            request_id,
        };

        Ok(Response::new(res))
    }
}
