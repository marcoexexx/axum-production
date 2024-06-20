use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
  pub id: i64,
  pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTaskInput {
  pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskInput {
  pub title: Option<String>,
}

pub struct TaskBmc;

impl TaskBmc {
  pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_input: CreateTaskInput) -> Result<i64> {
    let db = mm.db();

    let (id,) = sqlx::query_as::<_, (i64,)>("INSERT INTO task (title) values ($1) returning id")
      .bind(task_input.title)
      .fetch_one(db)
      .await?;

    Ok(id)
  }
}

#[cfg(test)]
mod tests {
  use crate::_dev_utils;

  #[allow(unused)]
  use super::*;
  use anyhow::Result;

  #[tokio::test]
  async fn test_task_create_ok() -> Result<()> {
    // -- Setup & Fixtures
    let mm = _dev_utils::init_test().await;
    let ctx = Ctx::root_ctx();
    let fx_title = "test_task_create_ok title";

    Ok(())
  }
}
