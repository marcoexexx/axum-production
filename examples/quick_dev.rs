use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8000")?;

  // hc.do_get("/index.html").await?.print().await?;

  hc.do_post(
    "/api/login",
    json!({
      "username": "demo1",
      "pwd": "welcome"
    }),
  )
  .await?
  .print()
  .await?;

  hc.do_post("/api/logout", json!({ "logout": true }))
    .await?
    .print()
    .await?;

  hc.do_get("/hello").await?.print().await?;

  Ok(())
}
