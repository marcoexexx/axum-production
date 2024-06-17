use axum::Router;
pub use error::Result;

use tokio::{net::TcpListener, signal};

mod error;

async fn shutdown_signal() {
  signal::ctrl_c().await.unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
  let routes = Router::new();

  let tcp_listener = TcpListener::bind("localhost:8000").await.unwrap();
  println!("🚀 Server is ready http://localhost:8000");

  axum::serve(tcp_listener, routes.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

  Ok(())
}
