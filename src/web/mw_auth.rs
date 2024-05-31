use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{ctx, Error, Result};

pub async fn mw_require_auth(
  ctx: Result<ctx::Ctx>,
  req: Request<Body>,
  next: Next,
) -> Result<Response> {
  println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

  ctx?;

  Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
  let (_whole, user_id, exp, sign) = regex_captures!(
    r#"^user-(\d+)\.(.+)\.(.+)"#, // a litteral regex,
    &token
  )
  .ok_or(Error::AuthFail(String::from("Invalid token format")))?;

  let user_id: u64 = user_id
    .parse()
    .map_err(|err| Error::AuthFail(format!("Invalid token from: {err:?}")))?;

  Ok((user_id, exp.to_string(), sign.to_string()))
}

// region:        ───── Ctx Extractor

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ctx::Ctx {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    println!("->> {:<12} - Ctx", "EXTRACTOR");

    // User the cookies extractor
    let cookies = parts.extract::<Cookies>().await.unwrap();

    let auth_token = cookies
      .get(AUTH_TOKEN)
      .map(|token| token.value().to_string());

    // Parse token
    let (user_id, exp, sign) = auth_token
      .ok_or(Error::AuthFail(String::from("invalid token")))
      .and_then(parse_token)?;

    // TODO: Token components validation

    Ok(ctx::Ctx::new(user_id))
  }
}

// endregion:     ───── Ctx Extractor
