use axum::async_trait;
use axum::extract::{FromRequestParts, Path};
use serde::de::DeserializeOwned;
use axum::http::request::Parts;
use validator::Validate;

use crate::helpers::util::utility::collect_error;

struct ParamValidator<T: Validate>(pub T);

#[async_trait]
impl<K, T> FromRequestParts<K> for ParamValidator<T>
    where
        K: Send + Sync,
        T: DeserializeOwned + Validate,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, state: &K) -> Result<Self, Self::Rejection> {
        let param_res = Path::<T>::from_request_parts(parts, state).await;

        if let Err(e) = param_res {
            return Err(e.into());
        }

        let Path(param_res) = param_res.unwrap();

        if let Err(e) = param_res.validate() {
            let error_message = collect_error(e).as_str();
            return Err(error_message.into());
        }

        Ok(ParamValidator(param_res))
    }
}