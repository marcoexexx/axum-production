use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::error::Result;
use crate::web;

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
  uuid: String,
  timestamp: String, // should be iso8601
  user_id: Option<i64>,
  http_path: String,
  http_method: String,
  client_error_type: Option<String>,
  error_type: Option<String>,
  error_data: Option<Value>,
}

pub async fn log_request(
  uuid: Uuid,
  req_method: Method,
  uri: Uri,
  ctx: Option<Ctx>,
  web_errror: Option<&web::error::Error>,
  client_error: Option<web::error::ClientError>,
) -> Result<()> {
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();

  let error_type = web_errror.map(|se| se.as_ref().to_string());
  let error_data = serde_json::to_value(web_errror)
    .ok()
    .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

  let log_line = RequestLogLine {
    uuid: uuid.to_string(),
    timestamp: timestamp.to_string(),
    http_path: uri.to_string(),
    http_method: req_method.to_string(),
    user_id: ctx.map(|c| c.user_id()),
    client_error_type: client_error.map(|e| e.as_ref().to_string()),
    error_data,
    error_type,
  };

  println!("->> REQUEST LOG LINE:\n{}", json!(log_line));

  Ok(())
}
