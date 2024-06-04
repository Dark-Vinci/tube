use {
    crate::util::client::{get_client, start_test_server},
    auth::config::config::Config,
    sdk::generated_proto_rs::tube_utils::Empty,
    tonic::Request,
};

#[tokio::test]
async fn ping() {
    let c_1 = Config::default();
    let c_2 = Config::default();

    let _ = start_test_server(c_1).await;
    let mut client = get_client(c_2).await;

    let request = Request::new(Empty {});

    let res = client.ping(request).await.unwrap();

    assert!(res.into_inner().message.len() > 0);
}
