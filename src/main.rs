use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service};
use axum::Router;
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// region:          ───── Router hello
fn routes_hello() -> Router {
  Router::new()
    .route("/hello2/:name", get(handle_hello2))
    .route("/hello", get(handle_hello))
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
// endregion:       ───── Router hello

// region:          ───── Handler hello
async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!("->> {:<12} - handle_hello - {params:?}", "HANDLER");

  let name = params.name.as_deref().unwrap_or("World");
  Html(format!("Hello <strong>{name}!!</strong>"))
}

async fn handle_hello2(Path(name): Path<String>) -> impl IntoResponse {
  println!("->> {:<12} - handle_hello2 - {name:?}", "HANDLER");

  Html(format!("Hello2 <strong>{name}</strong>"))
}
// endregion:       ───── Handler hello

#[tokio::main]
async fn main() {
  let routes_all = Router::new()
    .merge(routes_hello())
    .fallback_service(routes_static());

  // region:      ───── Start Server
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("==> LISTENING on {addr}\n");

  axum::Server::bind(&addr)
    .serve(routes_all.into_make_service())
    .await
    .unwrap();

  // endregion:   ───── Start Server
}
