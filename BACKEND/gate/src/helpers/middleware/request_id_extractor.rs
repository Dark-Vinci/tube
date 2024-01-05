use axum::async_trait;
use axum::extract::{FromRequest, Request};
use uuid::Uuid;

pub struct RequestId(pub Uuid);

#[async_trait]
impl<B> FromRequest<B> for RequestId
    where
        B: Send + Sync,
{
    type Rejection = String;

    async fn from_request(_req: Request, _state: &B) -> Result<Self, Self::Rejection> {
        let id = Uuid::new_v4();

        Ok(Self(id))
    }
}
