use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::base::{self, DbBmc};

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
  pub id: i64,
  pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct CreateTaskInput {
  pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct UpdateTaskInput {
  pub title: Option<String>,
}

pub struct TaskBmc;

impl DbBmc for TaskBmc {
  const TABLE: &'static str = "task";
}

impl TaskBmc {
  pub async fn create(ctx: &Ctx, mm: &ModelManager, task_input: CreateTaskInput) -> Result<i64> {
    base::create::<Self, _>(ctx, mm, task_input).await
  }

  pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
    base::get::<Self, _>(ctx, mm, id).await
  }

  pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
    base::list::<Self, _>(ctx, mm).await
  }

  pub async fn update(
    ctx: &Ctx,
    mm: &ModelManager,
    id: i64,
    task_input: UpdateTaskInput,
  ) -> Result<()> {
    base::update::<Self, _>(ctx, mm, id, task_input).await
  }

  pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
    base::delete::<Self>(ctx, mm, id).await
  }
}

#[cfg(test)]
mod tests {
  #![allow(unused)]

  use super::*;
  use crate::_dev_utils;
  use anyhow::Result;
  use serial_test::serial;

  #[serial]
  #[tokio::test]
  async fn test_create_ok() -> Result<()> {
    // -- Setup & Fixtures
    let mm = _dev_utils::init_test().await;
    let ctx = Ctx::root_ctx();
    let fx_title = "test_task_create_ok title";

    // -- Exec
    let create_task_input = CreateTaskInput {
      title: fx_title.to_string(),
    };
    let id = TaskBmc::create(&ctx, &mm, create_task_input).await?;

    // -- Check
    let task = TaskBmc::get(&ctx, &mm, id).await?;
    assert_eq!(task.title, fx_title);

    // -- Clean
    TaskBmc::delete(&ctx, &mm, id).await?;

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn test_get_err_not_found() -> Result<()> {
    // -- Setup & Fixtures
    let mm = _dev_utils::init_test().await;
    let ctx = Ctx::root_ctx();
    let fx_id = 100;

    // -- Exec
    let res = TaskBmc::get(&ctx, &mm, fx_id).await;

    // -- Check
    assert!(
      matches!(
        res,
        Err(Error::EntityNotFound {
          entity: "task",
          id: 100
        })
      ),
      "EntityNotFound not matching"
    );

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn test_list_ok() -> Result<()> {
    // -- Setup & Fixtures
    let mm = _dev_utils::init_test().await;
    let ctx = Ctx::root_ctx();
    let fx_titles = &["test_list_ok-test 01", "test_list_ok-test 02"];

    _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

    // -- Exec
    let tasks = TaskBmc::list(&ctx, &mm).await?;

    // -- Check
    let tasks: Vec<Task> = tasks
      .into_iter()
      .filter(|t| t.title.starts_with("test_list_ok-test"))
      .collect();
    assert_eq!(tasks.len(), fx_titles.len(), "number of seeded tasks.");

    // -- Clean
    for task in tasks {
      TaskBmc::delete(&ctx, &mm, task.id).await?;
    }

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn test_update_ok() -> Result<()> {
    let mm = _dev_utils::init_test().await;
    let ctx = Ctx::root_ctx();
    let fx_title = "test_update_ok - task 01";
    let fx_title_new = "test_update_ok - task 01 - new";
    let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
      .await?
      .remove(0);

    // -- Exec
    TaskBmc::update(
      &ctx,
      &mm,
      fx_task.id,
      UpdateTaskInput {
        title: Some(fx_title_new.to_string()),
      },
    )
    .await?;

    // -- Check
    let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
    assert_eq!(task.title, fx_title_new);

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn test_delete_err_not_found() -> Result<()> {
    // -- Setup & Fixtures
    let mm = _dev_utils::init_test().await;
    let ctx = Ctx::root_ctx();
    let fx_id = 100;

    // -- Exec
    let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

    // -- Check
    assert!(
      matches!(
        res,
        Err(Error::EntityNotFound {
          entity: "task",
          id: 100
        })
      ),
      "EntityNotFound not matching"
    );

    Ok(())
  }
}
