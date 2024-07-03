use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum::{response::Response, Json};
use modql::filter::ListOptions;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use serde_with::{serde_as, OneOrMany};
use std::sync::Arc;
use tracing::debug;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::rpc::task_rpc::{create_task, delete_task, list_tasks, update_task};
use crate::web::{Error, Result};

mod task_rpc;

#[derive(Deserialize)]
struct RpcRequest {
  id: Option<Value>,
  method: String,
  params: Option<Value>,
}

#[derive(Deserialize)]
struct ParamsCreateRequest<D> {
  data: D,
}

#[derive(Deserialize)]
struct ParamsUpdateRequest<D> {
  id: i64,
  data: D,
}

#[derive(Deserialize)]
struct ParamsIdedRequest {
  id: i64,
}

#[serde_as]
#[derive(Deserialize)]
struct ParamsListRequest<F>
where
  F: DeserializeOwned,
{
  #[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
  pub filters: Option<Vec<F>>,
  pub list_options: Option<ListOptions>,
}

pub fn routes(mm: ModelManager) -> Router {
  Router::new()
    .route("/rpc", post(rpc_handler))
    .with_state(mm)
}

async fn rpc_handler(
  State(mm): State<ModelManager>,
  ctx: Ctx,
  Json(rpc_req): Json<RpcRequest>,
) -> Response {
  let rpc_info = RpcInfo {
    id: rpc_req.id.clone(),
    method: rpc_req.method.clone(),
  };

  // -- Exec & Store RpcInfo in response.
  let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
  res.extensions_mut().insert(Arc::new(rpc_info));

  res
}

/// RPC basic information holding the id and method for further logging.
#[derive(Debug)]
pub struct RpcInfo {
  pub id: Option<Value>,
  pub method: String,
}

macro_rules! exec_rpc_fn {
  // With Params
  ($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
    let rpc_method_name = stringify!($rpc_fn);
    let params = $rpc_params.ok_or(Error::RpcMissingParams {
      rpc_method: rpc_method_name.to_string(),
    })?;
    let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
      rpc_method: rpc_method_name.to_string(),
    })?;
    $rpc_fn($ctx, $mm, params).await.map(to_value)??
  }};

  // Without Params
  ($rpc_fn:expr, $ctx:expr, $mm:expr) => {
    $rpc_fn($ctx, $mm).await.map(to_value)??
  };
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
  let RpcRequest {
    id: rpc_id,
    method: rpc_method,
    params: rpc_params,
  } = rpc_req;

  debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

  let result_json: Value = match rpc_method.as_str() {
    // -- Task RPC methods
    "create_task" => exec_rpc_fn!(create_task, ctx, mm, rpc_params),
    "list_tasks" => exec_rpc_fn!(list_tasks, ctx, mm, rpc_params),
    "update_task" => exec_rpc_fn!(update_task, ctx, mm, rpc_params),
    "delete_task" => exec_rpc_fn!(delete_task, ctx, mm, rpc_params),

    // -- Fallback as Err.
    _ => return Err(Error::RpcMethodUnknown(rpc_method)),
  };

  let body_response = json!({
    "id": rpc_id,
    "result": result_json
  });

  Ok(Json(body_response))
}
