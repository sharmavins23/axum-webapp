//! Defines the application's HTTP router.

use crate::routes::brew::route_brew;
use crate::routes::hello::route_hello;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

// ===== Definition ============================================================

/// Creates a router with all application routes.
pub fn create_router() -> Router {
    Router::new()
        .route("/brew/:name", get(route_brew))
        .route("/hello", get(route_hello))
        .layer(TraceLayer::new_for_http())
}
