use crate::web::{Error, Result, AUTH_TOKEN};

use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  pwd: String,
}

pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login_handler))
}

async fn api_login_handler(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
  debug!("{:<12} - api_login_handler", "HANDLER");

  // TODO: Implement real db/auth logic
  if payload.username != "demo1" || payload.pwd != "welcome" {
    return Err(Error::LoginFail);
  }

  // FIXME: Implement real auth-token generation/signature.
  cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

  // Create the success body.
  let body = Json(json!({
    "result": {
      "success": true
    }
  }));

  Ok(body)
}