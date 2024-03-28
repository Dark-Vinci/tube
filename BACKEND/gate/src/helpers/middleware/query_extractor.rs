use {
    crate::helpers::util::utility::collect_error,
    axum::{
        async_trait,
        extract::{FromRequestParts, Query},
        http::request::Parts,
    },
    serde::de::DeserializeOwned,
    validator::Validate,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct QueryValidator<T: Validate>(pub T);

#[async_trait]
impl<K, T> FromRequestParts<K> for QueryValidator<T>
where
    K: Send + Sync,
    T: DeserializeOwned + Validate + Clone + Send + Sync + Sized + 'static,
{
    type Rejection = String;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &K,
    ) -> Result<Self, Self::Rejection> {
        let query_res = Query::<T>::from_request_parts(parts, state).await;

        if let Err(e) = query_res {
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
