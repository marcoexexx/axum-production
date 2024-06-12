use self::error::Result;

pub mod error;

#[derive(Clone)]
pub struct ModelManager {}

impl ModelManager {
  pub async fn new() -> Result<Self> {
    Ok(ModelManager {})
  }
}
