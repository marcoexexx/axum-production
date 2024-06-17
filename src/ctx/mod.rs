pub use self::error::{Error, Result};

mod error;

#[derive(Debug, Clone)]
pub struct Ctx {
  user_id: i64,
}

impl Ctx {
  pub fn root_ctx() -> Self {
    Ctx { user_id: 0 }
  }

  pub fn try_new(user_id: i64) -> Result<Self> {
    if user_id == 0 {
      Err(Error::CtxCannotNewRootCtx)
    } else {
      Ok(Ctx { user_id })
    }
  }
}

impl Ctx {
  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}
