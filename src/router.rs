//! Defines the application's HTTP router.

use crate::routes::hello::route_hello;
use axum::{routing::get, Router};

// ===== Definition ============================================================

/// Creates a router with all application routes.
pub fn create_router() -> Router {
    Router::new().route("/hello", get(route_hello))
}
