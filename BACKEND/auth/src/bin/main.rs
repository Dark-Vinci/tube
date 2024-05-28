use {
    auth::{
        application::application::App, config::config::Config,
        controller::controller::Auth,
    },
    sdk::{
        constants::{
            helper::{
                LAGOS_TIME, LOCAL_HOST, LOG_DIR, LOG_FILE_NAME, LOG_WARNING_FILE_NAME,
                TIME_ZONE,
            },
            types::E,
        },
        generated_proto_rs::tube_auth::auth_service_server::AuthServiceServer,
        helpers::shutdown::graceful_shutdown,
    },
    std::{env, net::SocketAddr},
    tonic::transport::Server,
    tracing::debug,
    tracing_appender::rolling,
    tracing_core::LevelFilter,
    tracing_subscriber::{fmt::writer::MakeWriterExt, EnvFilter},
};

#[tokio::main]
async fn main() -> Result<(), E> {
    // set time zone
    env::set_var(TIME_ZONE, LAGOS_TIME);

    let debug_logger = rolling::never(LOG_DIR, LOG_FILE_NAME);
    let warning_error_logger = rolling::never(LOG_DIR, LOG_WARNING_FILE_NAME);

    let file_writer = debug_logger.and(warning_error_logger);

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?
        .add_directive("auth=debug".parse()?);

    tracing_subscriber::fmt()
        .pretty()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(file_writer)
        .with_env_filter(filter)
        .with_current_span(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    // load the config
    let config: Config = Default::default();

    let addr: SocketAddr = format!("{0}:{1}", LOCAL_HOST, &config.app_port).parse()?;

    let app_name = &config.app_name.clone();
    let service_name = &config.service_name.clone();

    // bootstrap application
    let app = App::new(config).await?;

    // bootstrap service controller
    let auth_server = Auth::new(app);

    debug!(
        "🚀{0} for {1} is listening on address {2} 🚀",
        app_name, service_name, addr
    );

    // start service and listen to shut down hooks;
    Server::builder()
        .add_service(AuthServiceServer::new(auth_server))
        .serve_with_shutdown(addr, graceful_shutdown())
        .await?;

    Ok(())
}
