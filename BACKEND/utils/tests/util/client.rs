use {
    auth::{
        application::application::App, config::config::Config,
        controller::controller::Auth,
    },
    sdk::{
        constants::helper::LOCAL_HOST,
        generated_proto_rs::tube_auth::{
            auth_service_client::AuthServiceClient,
            auth_service_server::AuthServiceServer,
        },
    },
    std::net::SocketAddr,
    tonic::transport::{Channel, Server},
};

pub async fn get_client(c: Config) -> AuthServiceClient<Channel> {
    let uri = format!("http://[::1]:{}", c.app_port);

    let client = AuthServiceClient::connect(uri).await.unwrap();

    return client;
}

pub async fn start_test_server(config: Config) {
    let addr: SocketAddr = format!("{0}:{1}", LOCAL_HOST, &config.app_port)
        .parse()
        .unwrap();

    let app = App::new(config).await.unwrap();

    // bootstrap service controller
    let auth_server = Auth::new(app);

    // start service and listen to shut down hooks;
    Server::builder()
        .add_service(AuthServiceServer::new(auth_server))
        .serve(addr)
        .await
        .unwrap()
}
