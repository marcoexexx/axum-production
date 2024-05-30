use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8000")?;

  hc.do_get("/hello?name=Marco").await?.print().await?;

  let post_body = json!({
      "username": "demo",
      "password": "demo",
  });
  hc.do_post("/api/auth/login", post_body)
    .await?
    .print()
    .await?;

  Ok(())
}
