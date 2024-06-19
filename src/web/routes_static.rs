use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::routing::{any_service, MethodRouter};
use tower_http::services::ServeDir;

use crate::config;

// Note: Here we can just return a MethodRouter rather then a full Router since ServeDir is a
//       service.
async fn handle_404() -> (StatusCode, &'static str) {
  (StatusCode::NOT_FOUND, "Resource not found")
}

pub fn serve_dir() -> MethodRouter {
  any_service(ServeDir::new(&config().WEB_FOLDER).not_found_service(handle_404.into_service()))
}
