pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  DataFailParse(String),
  FailToB64Decode,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl std::error::Error for Error {}
