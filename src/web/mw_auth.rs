use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::{Error, Result, AUTH_TOKEN};

use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub async fn mw_ctx_require(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
  debug!("{:<12} - mw_ctx_require", "MIDDLEWARE");

  ctx?;

  Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve(
  _mm: State<ModelManager>,
  cookies: Cookies,
  mut req: Request<Body>,
  next: Next,
) -> Result<Response> {
  debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

  let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

  // FIXME: thee cookie if something went wrong other then NoAuthTokenCookie.
  let result_ctx = Ctx::try_new(100).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()));

  if result_ctx.is_err() && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie)) {
    cookies.remove(Cookie::from(AUTH_TOKEN))
  }

  // Store the ctx_result in the request extension.
  req.extensions_mut().insert(result_ctx);

  Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    debug!("{:<12} - from_request_parts", "EXTRACTOR");

    parts
      .extensions
      .get::<CtxExtResult>()
      .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
      .clone()
      .map_err(Error::CtxExt)
  }
}

type CtxExtResult = std::result::Result<Ctx, CtxExtError>;

#[derive(Debug, Clone, Serialize)]
pub enum CtxExtError {
  TokenNotInCookie,
  CtxNotInRequestExt,
  CtxCreateFail(String),
}
