use axum::async_trait;
use axum::extract::{FromRequestParts, Query};
use serde::de::DeserializeOwned;
use axum::http::request::Parts;
use validator::Validate;

use crate::helpers::util::utility::collect_error;

#[derive(Debug, Clone, Copy, Default)]
pub struct QueryValidator<T: Validate>(pub T);

#[async_trait]
impl<K, T> FromRequestParts<K> for QueryValidator<T>
    where
        K: Send + Sync,
        T: DeserializeOwned + Validate + Clone + Send + Sync + Sized +'static
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, state: &K) -> Result<Self, Self::Rejection> {
        let query_res = Query::<T>::from_request_parts(parts, state).await;

        if let Err(e) = query_res {
            println!("{e}");
            return Err(e.to_string().as_str().parse().unwrap());
        }

        let Query(query_res) = query_res.unwrap();

        if let Err(err) = query_res.validate() {
            let error_message = collect_error(err);
            return Err(error_message.into());
        }

        Ok(QueryValidator(query_res))
    }
}
