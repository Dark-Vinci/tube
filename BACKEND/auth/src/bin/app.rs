use {
    auth::{
        application::application::App,
        config::config::Config,
        connections::{db::DBConnection, redis::Redis},
        controller::controller::Auth,
        downstream::downstream::DownStream,
        // migration::migrator::Migrator,
        repository::repository::Repo,
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
    // sea_orm_migration::{IntoSchemaManagerConnection, MigratorTrait},
    std::{env, net::SocketAddr},
    tonic::transport::Server,
    tracing::{debug, info},
    tracing_appender::rolling,
    tracing_subscriber::fmt::writer::MakeWriterExt,
};

#[tokio::main]
async fn main() -> Result<(), E> {
    // set time zone
    env::set_var(TIME_ZONE, LAGOS_TIME);

    let debug_logger = rolling::never(LOG_DIR, LOG_FILE_NAME);
    let warning_error_logger = rolling::never(LOG_DIR, LOG_WARNING_FILE_NAME);

    let file_writer = debug_logger.and(warning_error_logger);

    tracing_subscriber::fmt()
        .pretty()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(file_writer)
        .with_current_span(false)
        .init();

    info!("something should happen");

    // load the config
    let config = Config::new();

    let addr: SocketAddr = format!("{0}:{1}", LOCAL_HOST, &config.app_port).parse()?;

    // connect to necessary network services
    let db = DBConnection::open(&config).await?;
    let redis = Redis::connect(&config).await?;

    // if !&config.is_production {
    //     Migrator::up(db.0.into_schema_manager_connection(), None).await?;
    // }

    // using DB connection, bootstrap repository
    let repo = Repo::new(&db);

    // bootstrap the downstream
    let downstream = DownStream::new(&config).await?;

    let app_name = &config.app_name.clone();
    let service_name = &config.service_name.clone();

    // bootstrap application
    let app = App::new(config, downstream, repo, redis);

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
