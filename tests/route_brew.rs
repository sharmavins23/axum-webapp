//! Unit tests for the `/brew` route.

mod utils;

use color_eyre::eyre::Report;
use tokio::test;
use utils::server::create_server_client;

// ===== Tests =================================================================

/// Verify that the `/brew` endpoint returns "Brewing coffee for <name>...".
#[test]
async fn brew_alice() -> Result<(), Report> {
    let http_client = create_server_client().await?;

    // Verify that the `/brew/Alice` endpoint is working as expected
    let expected_response = "<h1>Brewing coffee for Alice...</h1>".to_string();
    let response = http_client.do_get("/brew/Alice").await?.text_body()?;
    assert_eq!(response, expected_response);

    Ok(())
}
