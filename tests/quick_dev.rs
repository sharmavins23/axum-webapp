#![allow(unused)] // Only for starting out!

// ===== Imports ===============================================================

use anyhow::{Ok, Result};

// ===== Test code =============================================================

/// Quick test to verify that the server is up and running.
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let http_client = httpc_test::new_client("http://localhost:8080")?;
    http_client.do_get("/hello").await?.print().await?;
    Ok(())
}
