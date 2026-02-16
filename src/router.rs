//! Defines the application's HTTP router.

use crate::routes::brew::route_brew;
use crate::routes::hello::route_hello;
use axum::{
    routing::{get, get_service},
    Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};

// ===== Helper functions ======================================================

/// Creates a static `/` endpoint that indexes into the file server.
fn static_index() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// ===== Definition ============================================================

/// Creates a router with all application routes.
pub fn create_router() -> Router {
    Router::new()
        .route("/brew/:name", get(route_brew))
        .route("/hello", get(route_hello))
        .layer(TraceLayer::new_for_http())
        .fallback_service(static_index())
}
