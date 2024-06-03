use tokio::net::TcpListener;

use self::error::{Error, Result};
use self::model::ModelController;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

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

async fn main_response_mapper(res: Response) -> Response {
  println!("->> {:12} - main_response_mapper", "RES_MAPPER");
  println!();

  res
}

#[tokio::main]
async fn main() -> Result<()> {
  let model_controller = ModelController::new().await?;

  let api_routes = web::routes_tickets::routes(model_controller.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

  let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", api_routes)
    .layer(middleware::map_response(main_response_mapper))
    .layer(middleware::from_fn_with_state(
      model_controller.clone(),
      web::mw_auth::mw_ctx_reolver,
    ))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

  // region:      ───── Start Server

  let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
  println!("==> LISTENING on {:?}\n", listener.local_addr());

  axum::serve(listener, routes_all.into_make_service())
    .await
    .unwrap();

  // endregion:   ───── Start Server

  Ok(())
}
