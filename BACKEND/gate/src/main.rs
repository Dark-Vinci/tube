use std::net::SocketAddr;

use tokio::net::TcpListener;
use axum::{Router, serve};
use axum::http::{StatusCode, Uri};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use tracing::{debug, error, trace};
// use tracing::log::trace;
// use tracing_subscriber::layer::SubscriberExt;
// use tracing_subscriber::util::SubscriberInitExt;

// use gate::helpers::util::graceful::graceful_shutdown;
// use gate::routes::routes::AppRouter;

#[tokio::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "example_global_404_handler=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_current_span(false)
        .init();

    trace!("what is wrong");

    // let app = AppRouter::routes();

    let app = Router::new().route("/", get(handler));

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = TcpListener::bind(&addr).await.unwrap();

    error!("listening on {}", listener.local_addr().unwrap());

    serve(listener, app)
        // .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap()

}

#[tracing::instrument(name="mango", ret)]
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tracing::instrument(name="tomato", ret)]
async fn handler_404(url: Uri) -> impl IntoResponse {
    let a: Result<(), &str> = Err("abc error");

    debug!(
        uri = url.to_string(),
        error = a.unwrap_err(),
        "__Got a handler 404__"
    );

    (StatusCode::NOT_FOUND, "nothing to see here")
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