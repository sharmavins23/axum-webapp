//! Main entry point for the application.

use axum::Server;
use axum_webapp::router::create_router;
use color_eyre::eyre::Report;
use std::net::SocketAddr;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// ===== Driver code ===========================================================

/// Main entry point for the application.
#[tokio::main]
async fn main() -> Result<(), Report> {
    // Initialize tracing/logging subscriber
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("tower_http=trace,axum_webapp=trace"))
        .init();

    let router = create_router();

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{address}");

    Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
