use std::fmt::Display;

use crate::model;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  // -- Model
  Model(model::Error),

  // -- Config
  ConfigMissingEnv(&'static str),
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
