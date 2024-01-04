use std::net::SocketAddr;

use tokio::net::TcpListener;
use axum::serve;
use tracing::{info, trace};

use gate::helpers::util::graceful::graceful_shutdown;
use gate::routes::routes::AppRouter;

#[allow(dead_code)]

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_current_span(false)
        .init();

    trace!("what is wrong");

    let app = AppRouter::routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("listening on 👂🏿👂🏿👂🏿{}", listener.local_addr().unwrap());

    serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap()

}

// #[derive(Debug, Deserialize, Default)]
// pub struct Pagination {
//     pub offset: Option<usize>,
//     pub limit: Option<usize>,
// }
// pagination: Option<Query<Pagination>>,
//

// #[derive(Debug, Deserialize)]
// struct CreateTodo {
//     text: String,
// }

// Json(input): Json<CreateTodo>

// #[derive(Debug, Deserialize)]
// struct UpdateTodo {
//     text: Option<String>,
//     completed: Option<bool>,
// }
//
// async fn todos_update(
//     Path(id): Path<Uuid>,
//     State(db): State<Db>,
//     Json(input): Json<UpdateTodo>,

// Path(id): Path<Uuid>,

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// struct Params {
//     #[serde(default, deserialize_with = "empty_string_as_none")]
//     foo: Option<i32>,
//     bar: Option<String>,
// }

// fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
//     where
//         D: Deserializer<'de>,
//         T: FromStr,
//         T::Err: fmt::Display,
// {
//     let opt = Option::<String>::deserialize(de)?;
//     match opt.as_deref() {
//         None | Some("") => Ok(None),
//         Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
//     }
// }