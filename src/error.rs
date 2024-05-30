use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  LoginFail,
  ResourceNotFound { id: u64 },
  InternalServerError,
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
    println!("->> {:<12} ───── {self}", "INTO_RESPONSE");

    match self {
      Self::LoginFail => (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response(),

      Self::ResourceNotFound { id } => (
        StatusCode::NOT_FOUND,
        format!("Not found resource id with `{id}`"),
      )
        .into_response(),

      Self::InternalServerError => (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unhandled internal server error",
      )
        .into_response(),
    }
  }
}
