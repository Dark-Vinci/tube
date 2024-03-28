// use crate::helpers::constants::constants::REQUEST_ID;
use {
    crate::{
        helpers::util::utility::collect_error,
        model::{
            error_response::AppError,
            response::{AppResponse, Data},
        },
    },
    axum::{
        async_trait,
        extract::{FromRequest, Request},
        response::Json as Rson,
        Json,
    },
    http::StatusCode,
    serde::de::DeserializeOwned,
    validator::Validate,
};

pub struct BodyValidator<T: Validate>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for BodyValidator<T>
where
    B: Send + Sync,
    T: DeserializeOwned + Validate + Clone + Send + Sync + Sized + 'static,
{
    type Rejection = Rson<AppResponse<Data>>;

    async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
        let b = Json::<T>::from_request(req, state).await;

        // let k = req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();

        if let Err(e) = b {
            let s = AppError::new(
                StatusCode::BAD_REQUEST,
                "INVALID STRUCT".to_string(),
                e.to_string(),
                "JSON parse error".to_string(),
                false,
            );
            let r = AppResponse::error(s, "".into(), StatusCode::BAD_REQUEST);

            return Err(Rson(r));
        }

        let Json(custom) = b.unwrap();

        if let Err(e) = custom.validate() {
            let error_message = collect_error(e);
            let s = AppError::new(
                StatusCode::BAD_REQUEST,
                "validation error".to_string(),
                error_message,
                "BodyValidator".to_string(),
                false,
            );
            let r = AppResponse::error(s, "".into(), StatusCode::BAD_REQUEST);

            return Err(Rson(r));
        }

        Ok(BodyValidator(custom))
    }
}
