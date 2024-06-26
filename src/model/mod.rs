//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `Modelmanager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controlers (e.g., `testBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `Modelmanager` are typically used as App State.
//! - Modelmanager are designed to be passed as an argument to all Model Controlers functions.

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

mod base;
mod error;
mod store;
pub mod task;
pub mod user;

#[derive(Clone)]
pub struct ModelManager {
  db: Db,
}

impl ModelManager {
  pub async fn new() -> Result<Self> {
    let db = new_db_pool().await?;

    Ok(ModelManager { db })
  }

  /// Return the sqlx db pool reference.
  /// (Only for the model layer)
  pub(in crate::model) fn db(&self) -> &Db {
    &self.db
  }
}
