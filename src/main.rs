#![allow(unused)] // For early local development

pub use self::config::config;
pub use self::error::{Error, Result};
use self::model::ModelManager;

pub mod _dev_utils; // Commented during early development.

use axum::{middleware, Router};
use tokio::{net::TcpListener, signal};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

async fn shutdown_signal() {
  signal::ctrl_c().await.unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt()
    .without_time() // For early local development
    .with_target(false)
    .with_env_filter(EnvFilter::from_default_env())
    .init();

  // -- FOR DEV ONLY
  _dev_utils::init_dev().await;

  let mm = ModelManager::new().await?;

  let routes = Router::new()
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(web::mw_res_map::mw_response_map))
    .layer(middleware::from_fn_with_state(
      mm.clone(),
      web::mw_auth::mw_ctx_resolve,
    ))
    .layer(CookieManagerLayer::new())
    .fallback_service(web::routes_static::serve_dir());

  let tcp_listener = TcpListener::bind("localhost:8000").await.unwrap();
  info!("🚀 Server is ready http://localhost:8000");

  axum::serve(tcp_listener, routes.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

  Ok(())
}
