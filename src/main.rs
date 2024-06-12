use axum::{middleware, Router};
use model::ModelManager;
use tokio::net::TcpListener;

use error::Result;
use tower_cookies::CookieManagerLayer;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
  let mm = ModelManager::new().await?;

  let routes = Router::new()
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(
      web::mw_response_map::mw_response_map,
    ))
    .layer(middleware::from_fn_with_state(
      mm.clone(),
      web::mw_auth::mw_ctx_resolver,
    ))
    .layer(CookieManagerLayer::new())
    .fallback_service(web::routes_static::serve_dir());

  let tcp_listener = TcpListener::bind("localhost:8000").await.unwrap();

  axum::serve(tcp_listener, routes.into_make_service())
    .await
    .unwrap();

  Ok(())
}
