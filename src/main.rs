#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![allow(unused)] // Only for starting out!

// ===== Imports ===============================================================

use axum::{response::Html, routing::get, Router, Server};
use std::net::SocketAddr;

// ===== Driver code ===========================================================

/// Main entry point for the application.
#[tokio::main]
async fn main() {
    let routes_hello =
        Router::new().route("/hello", get(|| async { Html("<h1>Hello, world!</h1>") }));

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{address}");

    Server::bind(&address)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
