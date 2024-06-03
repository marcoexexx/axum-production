use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::model::ModelController;
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

pub async fn mw_ctx_reolver(
  _model_controller: State<ModelController>,
  cookies: Cookies,
  mut req: Request<Body>,
  next: Next,
) -> Result<Response> {
  println!("->> {:12} - mw_ctx_reolver", "MIDDLEWARE");

  let auth_token = cookies
    .get(AUTH_TOKEN)
    .map(|token| token.value().to_string());

  // Compute Result<Ctx>
  let result_ctx = match auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token)
  {
    Ok((user_id, _exp, _sign)) => {
      // TODO: Token components validation
      Ok(ctx::Ctx::new(user_id))
    }
    Err(e) => Err(e),
  };

  // Remove the cookie if something went wrong
  if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
    cookies.remove(Cookie::from(AUTH_TOKEN))
  }

  // Store the result_ctx in the request expiration.
  req.extensions_mut().insert(result_ctx);

  Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
  let (_whole, user_id, exp, sign) = regex_captures!(
    r#"^user-(\d+)\.(.+)\.(.+)"#, // a litteral regex,
    &token
  )
  .ok_or(Error::AuthFailTokenWrongFormat)?;

  let user_id: u64 = user_id
    .parse()
    .map_err(|_| Error::AuthFailTokenWrongFormat)?;

  Ok((user_id, exp.to_string(), sign.to_string()))
}

// region:        ───── Ctx Extractor

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ctx::Ctx {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    println!("->> {:<12} - Ctx", "EXTRACTOR");

    // User the cookies extractor
    let ext_ctx = parts
      .extensions
      .get::<Result<ctx::Ctx>>()
      .ok_or(Error::AuthFailCtxNotInRequestExt)?;

    ext_ctx.clone()
  }
}

// endregion:     ───── Ctx Extractor
