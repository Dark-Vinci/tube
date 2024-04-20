use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("something went wrong, try again later")]
    GenericError,
}
