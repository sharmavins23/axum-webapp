#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

// ===== Imports ===============================================================

use axum::Server;
use axum_webapp::create_router;
use color_eyre::eyre::Report;
use std::net::SocketAddr;

// ===== Driver code ===========================================================

/// Main entry point for the application.
#[tokio::main]
async fn main() -> Result<(), Report> {
    let router = create_router();

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{address}");

    Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
