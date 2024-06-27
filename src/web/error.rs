use crate::{crypt, model, web};

use std::fmt::Display;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use tracing::debug;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
  // -- Login
  LoginFailUsernameNotFound,
  LoginFailUserHasNoPwd { user_id: i64 },
  LoginFailPwdNotMatching { user_id: i64 },

  // -- CtxExtError
  CtxExt(web::mw_auth::CtxExtError),

  // -- modules
  Model(model::Error),
  Crypt(crypt::Error),
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl std::error::Error for Error {}

impl From<model::Error> for Error {
  fn from(value: model::Error) -> Self {
    Self::Model(value)
  }
}

impl From<crypt::Error> for Error {
  fn from(value: crypt::Error) -> Self {
    Self::Crypt(value)
  }
}

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    debug!("{:<12} - model::Error {self:?}", "INTO_RES");

    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

    response.extensions_mut().insert(Arc::new(self));
    response
  }
}

impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    match self {
      // -- Login
      Self::LoginFailUsernameNotFound
      | Self::LoginFailUserHasNoPwd { .. }
      | Self::LoginFailPwdNotMatching { .. } => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

      // --Auth
      Self::CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

      // -- Fallback
      _ => (
        StatusCode::INTERNAL_SERVER_ERROR,
        ClientError::SERVICE_ERROR,
      ),
    }
  }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
  LOGIN_FAIL,
  NO_AUTH,
  SERVICE_ERROR,
}
