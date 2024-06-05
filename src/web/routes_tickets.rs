// region:          ───── RESR Handler

use axum::extract::Path;
use axum::extract::State;
use axum::routing::{delete, post};
use axum::Json;
use axum::Router;

use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketInput};
use crate::Result;

pub fn routes(model_controller: ModelController) -> Router {
  Router::new()
    .route("/tickets", post(create_ticket).get(find_many_ticket))
    .route("/tickets/:id", delete(delete_ticket))
    .with_state(model_controller)
}

async fn create_ticket(
  State(model_controller): State<ModelController>,
  ctx: Ctx,
  Json(ticket_input): Json<TicketInput>,
) -> Result<Json<Ticket>> {
  println!("->> {:<12} - create_ticket", "HANDLER");

  let ticket = model_controller.create_ticket(ctx, ticket_input).await?;

  Ok(Json(ticket))
}

async fn find_many_ticket(
  State(model_controller): State<ModelController>,
  ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
  println!("->> {:<12} - find_many_ticket", "HANDLER");

  let tickets = model_controller.find_many(ctx).await?;

  Ok(Json(tickets))
}

async fn delete_ticket(
  State(model_controller): State<ModelController>,
  ctx: Ctx,
  Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
  println!("->> {:<12} - delete_ticket", "HANDLER");

  let deleted_ticket = model_controller.delete_ticket(ctx, id).await?;

  Ok(Json(deleted_ticket))
}

// endregion:       ───── RESR Handler
