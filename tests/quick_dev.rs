use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8000")?;

  hc.do_get("/hello?name=Marco").await?.print().await?;

  let req_post_body = hc.do_post(
    "/api/auth/login",
    json!({
        "username": "demo",
        "password": "demo",
    }),
  );
  req_post_body.await?.print().await?;

  hc.do_post("/api/tickets", json!({ "title": format!("Ticket ##") }))
    .await?
    .print()
    .await?;

  hc.do_get("/api/tickets").await?.print().await?;

  hc.do_get("/api/tickets/detail").await?.print().await?;

  Ok(())
}
