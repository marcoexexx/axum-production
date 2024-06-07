#[derive(Clone, Debug)]
pub struct Ctx {
  user_id: u64,
}

impl Ctx {
  pub fn new(user_id: u64) -> Ctx {
    Ctx { user_id }
  }
}

// Property Accessors
impl Ctx {
  pub fn user_id(&self) -> u64 {
    self.user_id
  }
}