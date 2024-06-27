use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::user::{LoginUserInput, UserBmc};
use crate::model::ModelManager;
use crate::web::{self, remove_token_cookie, Error, Result, AUTH_TOKEN};

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  pwd: String,
}

pub fn routes(mm: ModelManager) -> Router {
  Router::new()
    .route("/api/login", post(api_login_handler))
    .route("/api/logout", post(api_logout_handler))
    .with_state(mm)
}

async fn api_login_handler(
  State(mm): State<ModelManager>,
  cookies: Cookies,
  Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
  debug!("{:<12} - api_login_handler", "HANDLER");

  let LoginPayload {
    username,
    pwd: pwd_clear,
  } = payload;

  let root_ctx = Ctx::root_ctx();

  // -- Get the user
  let user: LoginUserInput = UserBmc::first_by_username(&root_ctx, &mm, &username)
    .await?
    .ok_or(Error::LoginFailUsernameNotFound)?;
  let user_id = user.id;

  // -- Validate the password.
  let Some(pwd) = user.pwd else {
    return Err(Error::LoginFailUserHasNoPwd { user_id })
  };

  pwd::validate_pwd(
    &EncryptContent {
      content: pwd_clear.clone(),
      salt: user.pwd_salt.to_string(),
    },
    &pwd,
  )
  .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

  // -- Set web token.
  web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

  // Create the success body.
  let body = Json(json!({
    "result": {
      "success": true
    }
  }));

  Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
  logout: bool,
}

async fn api_logout_handler(
  cookies: Cookies,
  Json(payload): Json<LogoutPayload>,
) -> Result<Json<Value>> {
  debug!("{:<12} - api_logout_handler", "HANDLER");
  let should_logout = payload.logout;

  if should_logout {
    remove_token_cookie(&cookies)?;
  }

  let body = Json(json!({
    "result": { "logged out": payload.logout }
  }));

  Ok(body)
}
