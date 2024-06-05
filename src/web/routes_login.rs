use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{cookie, Cookies};

use crate::{web, Error, Result};

pub fn routes() -> Router {
  Router::new().route("/api/auth/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  password: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
  println!("->> {:<12} - api_login", "HANDLER");

  // TODO: Implement real db/auth logic.
  if payload.username != "demo" || payload.password != "demo" {
    return Err(Error::LoginFail);
  }

  // FIXME: Implement real auth-token generation/signature.
  let auth_cookie = cookie::Cookie::build((web::AUTH_TOKEN, "user-1.exp.sign"))
    .http_only(true)
    .secure(true)
    .max_age(cookie::time::Duration::seconds(60 * 15))
    .same_site(cookie::SameSite::Lax)
    .path("/")
    .build();

  cookies.add(auth_cookie);

  // Create the success body.
  let body = Json(json!({
    "result": {
      "success": true
    }
  }));

  Ok(body)
}
