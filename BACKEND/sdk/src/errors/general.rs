use thiserror::Error;

#[derive(Debug, Error, Default)]
pub enum GenericError {
    #[error("{field_name} with property {id} object is not found")]
    NotFound { id: String, field_name: String },

    #[default]
    #[error("It is not you, it is use. Try again later")]
    ServerError,
}
