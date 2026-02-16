//! Route handler for the `/brew` endpoint.
//!
//! # Path Parameters
//! - `name`: The name of the person to brew coffee for.
//!
//! # Examples
//! - `/brew/Alice` returns "Brewing coffee for Alice..."
//! - `/brew/Bob` returns "Brewing coffee for Bob..."

use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};

// ===== Handler ===============================================================

/// Handler for the `/brew` route.
pub async fn route_brew(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("<h1>Brewing coffee for {name}...</h1>"))
}
