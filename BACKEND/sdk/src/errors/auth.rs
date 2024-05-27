use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthAppError {
    #[error("the provided token has expired")]
    AuthTokenExpired,
}
