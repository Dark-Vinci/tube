use {
    crate::helpers::constants::constants::REQUEST_ID,
    axum::{
        async_trait,
        extract::{FromRequest, FromRequestParts, Request},
    },
    http::request::Parts,
    std::str::FromStr,
    uuid::Uuid,
};

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

// GET THE REQUEST ID
pub struct GetRequestID(pub Uuid);

#[async_trait]
impl<B> FromRequest<B> for GetRequestID
where
    B: Send + Sync,
{
    type Rejection = String;

    async fn from_request(req: Request, _state: &B) -> Result<Self, Self::Rejection> {
        // we can control this, so no need for error handling, we'll always generate a UUID
        let i = req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();

        // we can also still control here too;
        let k = Uuid::from_str(i).unwrap();

        Ok(Self(k))
    }
}

// SET THE REQUEST ID
pub struct RequestID;

#[async_trait]
impl<B> FromRequestParts<B> for RequestID
where
    B: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &B,
    ) -> Result<Self, Self::Rejection> {
        let id = Uuid::new_v4();

        parts
            .headers
            .insert(REQUEST_ID, id.to_string().parse().unwrap());

        Ok(Self)
    }
}
