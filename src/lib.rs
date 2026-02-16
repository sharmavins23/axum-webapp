use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

// ===== Router Instance =======================================================

/// Creates a Router instance.
pub fn create_router() -> Router {
    Router::new().route("/hello", get(handler_hello))
}

// ===== Handler functions =====================================================

/// Handler for the `/hello` route.
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("<h1>Hello, world!</h1>")
}
