use axum::Router;
pub use error::Result;

use tokio::{net::TcpListener, signal};
use tower_cookies::CookieManagerLayer;

mod error;
mod web;

async fn shutdown_signal() {
  signal::ctrl_c().await.unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
  let routes = Router::new()
    .merge(web::routes_login::routes())
    .layer(CookieManagerLayer::new())
    .fallback_service(web::routes_static::serve_dir());

  let tcp_listener = TcpListener::bind("localhost:8000").await.unwrap();
  println!("🚀 Server is ready http://localhost:8000");

  axum::serve(tcp_listener, routes.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

  Ok(())
}
