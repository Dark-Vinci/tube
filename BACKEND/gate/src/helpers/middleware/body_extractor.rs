use axum::{async_trait, Json};
use axum::extract::{FromRequest, Request};
use axum::response::IntoResponse;
use axum::body::{Body, Bytes};
use serde::de::DeserializeOwned;
use validator::Validate;
// use axum::Json;

use crate::helpers::util::utility::collect_error;


pub struct BodyValidator<T: Validate>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for BodyValidator<T>
    where
        B: Send + Sync,
        T: DeserializeOwned + Validate + Clone + Send + Sync + Sized +'static
{
    type Rejection = String;

    async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
        let b = Json::<T>::from_request(req, state).await;

        if let Err(e) = b {
            println!("{e}");
            return Err(e.to_string());
        }

        let Json(custom) = b.unwrap();

        if let Err(e) = custom.validate() {
            let error_message = collect_error(e);
            return Err(error_message)
        }

        Ok(BodyValidator(custom))
    }
}
