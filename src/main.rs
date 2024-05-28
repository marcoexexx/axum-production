use std::net::SocketAddr;

use axum::{
  response::{Html, IntoResponse}, routing::get, Router
};

async fn handle_hello() -> impl IntoResponse {
  println!("->> {:<12} - handle_hello", "HANDLER");

  Html("Hello <strong>World!!</strong>")
}

#[tokio::main]
async fn main() {
  let routes_hello = Router::new().route("/hello", get(handle_hello));

  // region:      ───── Start Server
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("==> LISTENING on {addr}\n");

  axum::Server::bind(&addr)
    .serve(routes_hello.into_make_service())
    .await
    .unwrap();

  // endregion:   ───── Start Server
}
