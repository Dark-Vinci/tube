use {fred::error::RedisError, sea_orm::DbErr, thiserror::Error};

#[derive(Debug, Error, Default)]
pub enum GenericError {
    #[error("{field_name} with property {id} object is not found")]
    NotFound { id: String, field_name: String },

    #[default]
    #[error("It is not you, it is use. Try again later")]
    ServerError,
}

#[derive(Debug, Error, Default)]
pub enum GRPCError {
    #[error("Could not connect to {0}")]
    UnableToConnect(String),

    #[default]
    #[error("still donr know")]
    IDontKnow,
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("DB connection error: {0}")]
    DB(#[from] DbErr),

    #[error("rabbitMq connection error: {0}")]
    Rabbit(#[from] lapin::Error),

    #[error("redis connection error: {0}")]
    Redis(#[from] RedisError),
}

#[derive(Debug, Error)]
pub enum DBError {
    #[error("Entity {0} with property {1} not found")]
    NotFound(String, String),
}

pub enum GenericApplicationError {
    Validation,
    Serialization,
    InvalidUUID,
}

pub enum DownstreamError {
    Connection,
}
