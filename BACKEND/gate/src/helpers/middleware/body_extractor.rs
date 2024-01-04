use axum::async_trait;
use axum::extract::{FromRequest, Request};
use axum::response::IntoResponse;
use axum::body::Bytes;
use serde::de::DeserializeOwned;
// use tracing::error;
use validator::Validate;

use crate::helpers::util::utility::collect_error;


pub struct BodyValidator<T: Validate>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for BodyValidator<T>
    where
        B: Send + Sync,
        T: DeserializeOwned + Validate
{
    type Rejection = String;

    async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response).unwrap();

        // Parse the body bytes into your custom data structure
        let custom_data: Result<T, &str> = serde_json::from_slice(&*body)
            .map_err(|e| e.to_string().as_str());

        if let Err(e) = custom_data {
            println!("{e}");
            return Err(e.into());
        }

        let custom = custom_data.unwrap();

        if let Err(e) = custom.validate() {
            let error_message = collect_error(e).as_str();
            return Err(error_message.into())
        }


        Ok(BodyValidator(custom))
    }
}
