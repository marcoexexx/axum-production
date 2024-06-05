use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
  LoginFail,

  AuthFailNoAuthTokenCookie,
  AuthFailTokenWrongFormat,
  AuthFailCtxNotInRequestExt,

  ResourceNotFound { id: u64 },
}

// region:          ───── Error boilerplate

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error From Display trait {self:?}")
  }
}

impl std::error::Error for Error {}

// endregion:       ───── Error boilerplate

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    println!("    ->> {:<12} ───── {self}", "INTO_RESPONSE");

    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

    // Insert the error into the response
    response.extensions_mut().insert(self);

    response
  }
}

impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    #[allow(unreachable_patterns)]
    match self {
      Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LoginFail),

      // -- Auth
      Self::AuthFailTokenWrongFormat
      | Self::AuthFailNoAuthTokenCookie
      | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NoAuth),

      // -- Model
      Self::ResourceNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::InvalidParams),

      // -- Fallback
      _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::ServiceError),
    }
  }
}

#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
  LoginFail,
  NoAuth,
  InvalidParams,
  ServiceError,
}
