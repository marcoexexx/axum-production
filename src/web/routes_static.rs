use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::routing::{any_service, MethodRouter};
use tower_http::services::ServeDir;

const WEB_FOLDER: &str = "public";

async fn handle_404() -> (StatusCode, &'static str) {
  (StatusCode::NOT_FOUND, "Resource not found")
}

pub fn serve_dir() -> MethodRouter {
  any_service(ServeDir::new(WEB_FOLDER).not_found_service(handle_404.into_service()))
}
