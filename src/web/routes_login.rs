use crate::web::error::{Error, Result};
use crate::web::AUTH_TOKEN;

use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  password: String,
}

pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login_handler))
}

async fn api_login_handler(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
  if payload.username != "demo1" || payload.password != "welcome" {
    return Err(Error::LoginFail);
  }

  cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

  let body = Json(json!({
    "result": { "success": true}
  }));

  Ok(body)
}
