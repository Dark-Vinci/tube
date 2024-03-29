use {
    crate::application::traits::Application,
    sdk::generated_proto_rs::{
        tube_auth::{
            auth_service_server::AuthService, PingResponse, SayHelloRequest,
            SayHelloResponse,
        },
        tube_utils::Empty,
    },
    std::sync::Arc,
    tonic::{async_trait, Request, Response, Status},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct Auth<T: Application>(Arc<T>);

impl<T: Application> Auth<T> {
    pub fn new(a: T) -> Self {
        Self(Arc::new(a))
    }
}

#[async_trait]
impl<T: Application + Send + Sync + 'static> AuthService for Auth<T> {
    async fn ping(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<PingResponse>, Status> {
        let id = Uuid::new_v4();
        let res = PingResponse {
            message: self.0.ping(id),
            // message: "self.0.ping(id)".into(),
        };

        Ok(Response::new(res))
    }

    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        let SayHelloRequest { name, request_id } = request.into_inner();

        let res = SayHelloResponse {
            message: format!("Hello {} from say hell method", name).to_string(),
            request_id,
        };

        Ok(Response::new(res))
    }
}
