//! Route handler for the `/hello` endpoint.

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

// ===== Route parameters ======================================================

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

// ===== Handler ===============================================================

/// Handler for the `/hello` route.
pub async fn route_hello(params: Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - route_hello - {params:?}", "HANDLER");

    let name: &str = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello, {name}!</h1>"))
}
