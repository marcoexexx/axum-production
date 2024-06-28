use crate::ctx::Ctx;
use crate::model::task::{CreateTaskInput, Task, TaskBmc, UpdateTaskInput};
use crate::model::ModelManager;
use crate::web::Result;

use super::{ParamsCreateRequest, ParamsIdedRequest, ParamsUpdateRequest};

pub async fn create_task(
  ctx: Ctx,
  mm: ModelManager,
  params: ParamsCreateRequest<CreateTaskInput>,
) -> Result<Task> {
  let ParamsCreateRequest { data } = params;

  let id = TaskBmc::create(&ctx, &mm, data).await?;
  let task = TaskBmc::get(&ctx, &mm, id).await?;

  Ok(task)
}

pub async fn list_tasks(ctx: Ctx, mm: ModelManager) -> Result<Vec<Task>> {
  let tasks = TaskBmc::list(&ctx, &mm, None, None).await?;

  Ok(tasks)
}

pub async fn update_task(
  ctx: Ctx,
  mm: ModelManager,
  params: ParamsUpdateRequest<UpdateTaskInput>,
) -> Result<Task> {
  let ParamsUpdateRequest { id, data } = params;

  TaskBmc::update(&ctx, &mm, id, data).await?;

  let task = TaskBmc::get(&ctx, &mm, id).await?;

  Ok(task)
}

pub async fn delete_task(ctx: Ctx, mm: ModelManager, params: ParamsIdedRequest) -> Result<Task> {
  let ParamsIdedRequest { id } = params;

  let task = TaskBmc::get(&ctx, &mm, id).await?;
  TaskBmc::delete(&ctx, &mm, id).await?;

  Ok(task)
}
