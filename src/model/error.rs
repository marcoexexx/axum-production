use std::fmt::Display;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::crypt;

use super::store;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
  EntityNotFound { entity: &'static str, id: i64 },

  // -- Modules
  Crypt(crypt::Error),
  Store(store::Error),

  // -- Externals
  Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl From<store::Error> for Error {
  fn from(value: store::Error) -> Self {
    Self::Store(value)
  }
}

impl From<crypt::Error> for Error {
  fn from(value: crypt::Error) -> Self {
    Self::Crypt(value)
  }
}

impl From<sqlx::Error> for Error {
  fn from(value: sqlx::Error) -> Self {
    Self::Sqlx(value)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl std::error::Error for Error {}
