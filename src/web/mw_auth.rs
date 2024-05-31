use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
  let auth_token = cookies
    .get(AUTH_TOKEN)
    .map(|cookie| cookie.value().to_string());

  println!(
    "->> {:<12} - mw_require_auth - {auth_token:?}",
    "MIDDLEWARE"
  );

  // TODO: Real auth-token parsing & validation
  auth_token.ok_or(Error::LoginFail(String::from("invalid token")))?;

  Ok(next.run(req).await)
}
