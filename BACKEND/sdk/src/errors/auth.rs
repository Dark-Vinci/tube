use {thiserror::Error};

#[derive(Debug, Error)]
enum AuthAppError {
    #[error("the provided token has expired")]
    AuthTokenExpired,
}