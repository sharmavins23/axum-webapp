//! Re-usable utility for quickly setting up a test server/client.

use axum::Server;
use axum_webapp::router::create_router;
use color_eyre::eyre::Report;
use httpc_test::{new_client, Client};
use std::{
    net::{SocketAddr, TcpListener},
    time::Duration,
};
use tokio::{spawn, time::sleep};

// ===== Helper functions ======================================================

/// Runs the server using the provided TCP listener.
async fn run_server(listener: TcpListener) {
    let server = match Server::from_tcp(listener) {
        Ok(server) => server,
        Err(e) => {
            eprintln!("->> ERROR - Failed to start server: {e}");
            return;
        }
    };

    let service = create_router().into_make_service();
    if let Err(e) = server.serve(service).await {
        eprintln!("->> ERROR - Server error: {e}");
    }
}

/// Spawns the server in the background and returns the listening address.
fn spawn_server(addr: &str) -> Result<SocketAddr, Report> {
    let listener: TcpListener = TcpListener::bind(addr)?;
    // Set to non-blocking mode so we can spawn the server in the background
    listener.set_nonblocking(true)?;
    let address: SocketAddr = listener.local_addr()?;

    spawn(run_server(listener));

    Ok(address)
}

// ===== Client creation =======================================================

/// Creates an HTTP client connected to the server.
pub async fn create_server_client() -> Result<Client, Report> {
    // Bind to a random port for each test run to avoid conflicts
    let address: SocketAddr = spawn_server("127.0.0.1:0")?;
    let url: String = format!("http://{}", address);

    // Wait for the server to start up before creating the client
    sleep(Duration::from_millis(100)).await;

    Ok(new_client(&url)?)
}
