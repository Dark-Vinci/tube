// GLOBALly available constants
pub const APP_NAME: &'static str = "tube clone";
pub const LOCAL_HOST: &'static str = "[::1]";
pub const TIME_ZONE: &'static str = "TZ";
pub const LAGOS_TIME: &'static str = "Africa/Lagos";
pub const IS_PRODUCTION: &'static str = "IS_PRODUCTION";
pub const FALSE: &'static str = "false";

// default values
pub const DEFAULT_DB_PORT_VALUE: &'static str = "5420";
pub const DEFAULT_DB_HOST_VALUE: &'static str = "localhost";
pub const DEFAULT_DB_USERNAME_VALUE: &'static str = "tomiwa";
pub const DEFAULT_DB_PASSWORD_VALUE: &'static str = "tomiwa";
pub const DEFAULT_DB_AUTH_VALUE: &'static str = "auth";

// store constants
pub const DB_URL: &'static str = "DB_URL";
pub const DB_HOST: &'static str = "DB_HOST";
pub const DB_PORT: &'static str = "DB_PORT";
pub const DB_USERNAME: &'static str = "DB_USERNAME";
pub const DB_PASSWORD: &'static str = "DB_PASSWORD";
pub const REDIS_NAME: &'static str = "REDIS_NAME";
pub const REDIS_PASSWORD: &'static str = "REDIS_PASSWORD";
pub const REDIS_USERNAME: &'static str = "REDIS_USERNAME";
pub const REDIS_HOST: &'static str = "REDIS_HOST";
pub const REDIS_PORT: &'static str = "REDIS_PORT";
pub const REDIS_POOL_SIZE: &'static str = "REDIS_POOL_SIZE";
pub const DEFAULT_REDIS_CONNECTION_POOL: &'static str = "8";

// log constants
pub const LOG_DIR: &'static str = "./logs";
pub const LOG_FILE_NAME: &'static str = "logger.log";
pub const LOG_WARNING_FILE_NAME: &'static str = "problems.log";

// This is for AUTH related RPC server
pub const AUTH_PORT: &'static str = "50551";
pub const REACTION_URL: &'static str = "REACTION_URL";
pub const AUTH_DB_NAME: &'static str = "AUTH_DB_NAME";
pub const AUTH_NAME: &'static str = "auth GRPC server";
