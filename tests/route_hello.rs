//! Unit tests for the `/hello` route.

mod utils;

use color_eyre::eyre::Report;
use utils::server::create_server_client;

// ===== Tests =================================================================

/// Verify that the `/hello?name=Alice` endpoint returns "Hello, Alice!".
#[tokio::test]
async fn hello_alice() -> Result<(), Report> {
    let http_client = create_server_client().await?;

    // Verify that the `/hello?name=Alice` endpoint is working as expected
    let expected_response = "<h1>Hello, Alice!</h1>".to_string();
    let response = http_client.do_get("/hello?name=Alice").await?.text_body()?;
    assert_eq!(response, expected_response);

    Ok(())
}

/// Verify that the `/hello` endpoint returns "Hello, World!".
#[tokio::test]
async fn hello_world() -> Result<(), Report> {
    let http_client = create_server_client().await?;

    // Verify that the `/hello` endpoint is working as expected
    let expected_response = "<h1>Hello, World!</h1>".to_string();
    let response = http_client.do_get("/hello").await?.text_body()?;
    assert_eq!(response, expected_response);

    Ok(())
}
