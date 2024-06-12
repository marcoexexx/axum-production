use std::fmt::Display;

use crate::model;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  // -- Modules
  Model(model::error::Error),
}

impl From<model::error::Error> for Error {
  fn from(value: model::error::Error) -> Self {
    Self::Model(value)
  }
}

impl std::error::Error for Error {}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}
