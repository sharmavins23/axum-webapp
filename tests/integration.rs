//! Quick integration test to verify that the server is up and running.
//! Spawns the server in the background and makes a test HTTP request.

use axum::Server;
use axum_webapp::create_router;
use color_eyre::eyre::Report;
use httpc_test::Client;
use std::{
    net::{SocketAddr, TcpListener},
    time::Duration,
};
use tokio::time;

// ===== Helper functions ======================================================

/// Spawns the server in the background and returns the listening address.
fn spawn_server(addr: &str) -> Result<SocketAddr, Report> {
    let listener: TcpListener = TcpListener::bind(addr)?;
    // Set to non-blocking mode so we can spawn the server in the background
    listener.set_nonblocking(true)?;
    let address: SocketAddr = listener.local_addr()?;

    tokio::spawn(async move {
        match Server::from_tcp(listener) {
            Ok(server) => {
                if let Err(e) = server.serve(create_router().into_make_service()).await {
                    eprintln!("->> ERROR - Server error: {e}");
                }
            }
            Err(e) => {
                eprintln!("->> ERROR - Failed to start server: {e}");
            }
        }
    });

    Ok(address)
}

/// Waits for a moment to allow the server to start up.
async fn wait_for_server_startup() {
    time::sleep(Duration::from_millis(100)).await;
}

// ===== Test code =============================================================

/// Entry point for the integration test.
#[tokio::test]
async fn integration() -> Result<(), Report> {
    let address: SocketAddr = spawn_server("127.0.0.1:8080")?;
    let url: String = format!("http://{}", address);

    wait_for_server_startup().await;
    println!("->> TEST - Server is listening on {url}");

    let http_client: Client = httpc_test::new_client(&url)?;
    http_client.do_get("/hello").await?.print().await?;

    Ok(())
}
