use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::error::{Error, Result};
use crate::web::AUTH_TOKEN;

use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, body::Body};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

type CtxExtResult = std::result::Result<Ctx, CtxExtError>;

#[derive(Debug, Clone, Serialize)]
pub enum CtxExtError {
  TokenNotInCookie,
  CtxNotInRequestExt,
  CtxCreateFail(String),
}

pub async fn mw_ctx_require(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
  ctx?;

  Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
  _mm: State<ModelManager>,
  cookies: Cookies,
  mut req: Request<Body>,
  next: Next,
) -> Result<Response> {
  let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.to_string());

  // FIXME: Compute real CtxAuthResult<Ctx>
  let result_ctx = Ctx::new(100).map_err(|err| CtxExtError::CtxCreateFail(err.to_string()));

  if result_ctx.is_ok() && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie)) {
    cookies.remove(Cookie::from(AUTH_TOKEN))
  }

  req.extensions_mut().insert(result_ctx);

  Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    parts
      .extensions
      .get::<CtxExtResult>()
      .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
      .clone()
      .map_err(Error::CtxExt)
  }
}
