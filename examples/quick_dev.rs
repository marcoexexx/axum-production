use anyhow::Result;
use serde_json::json;

macro_rules! print {
  ($hc:expr, $url:expr, $data:expr) => {
    $hc.do_post($url, $data).await?.print().await?;
  };
}

#[tokio::main]
async fn main() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8000")?;

  // hc.do_get("/index.html").await?.print().await?;

  print!(
    hc,
    "/api/login",
    json!({ "username": "demo1", "pwd": "welcome" })
  );

  print!(
    hc,
    "/api/rpc",
    json!({
      "id": 1,
      "method":
      "create_task",
      "params": {
        "data": {
          "title": "task AAA"
        }
      }
    })
  );

  // print!(
  //   hc,
  //   "/api/rpc",
  //   json!({
  //     "id": 1,
  //     "method":
  //     "update_task",
  //     "params": {
  //       "id": 1000,
  //       "data": {
  //         "title": "task BBB"
  //       }
  //     }
  //   })
  // );

  // print!(
  //   hc,
  //   "/api/rpc",
  //   json!({
  //     "id": 1,
  //     "method": "delete_task",
  //     "params": {
  //       "id": 1000,
  //     }
  //   })
  // );

  print!(
    hc,
    "/api/rpc",
    json!({
      "id": 1,
      "method": "list_tasks",
      "params": {
        "list_options": {
          "order_by": "!id"
        }
      }
    })
  );

  // hc.do_post("/api/logout", json!({ "logout": true }))
  //   .await?
  //   .print()
  //   .await?;

  Ok(())
}
