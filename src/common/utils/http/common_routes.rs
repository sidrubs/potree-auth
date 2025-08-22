//! A set of commonly used "admin" style routes that should be present on a
//! production server.

use std::sync::LazyLock;

use axum::Router;
use axum::routing::get;
use http::StatusCode;
use web_route::WebRoute;

static HEALTH: LazyLock<WebRoute> = LazyLock::new(|| WebRoute::new("/_health"));

pub fn build_router() -> Router {
    Router::new().route(&HEALTH, get(health_check))
}

/// Responds with an empty 200. Used to check if the server is ready to accept
/// requests.
async fn health_check() -> StatusCode {
    StatusCode::OK
}
