use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("its not you, it is us")]
    SomethingWentWrong,
}
