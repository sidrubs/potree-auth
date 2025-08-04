use std::sync::LazyLock;

use axum::Extension;
use axum::Router;
use axum::routing::get;
use web_route::WebRoute;

use super::super::application::service::AuthenticationService;
use super::routes;
use super::state::State;

static LOGIN: LazyLock<WebRoute> = LazyLock::new(|| WebRoute::new("/login"));
pub static CALLBACK: LazyLock<WebRoute> = LazyLock::new(|| WebRoute::new("/callback"));

/// Builds a router that performs OIDC authentication.
///
/// **Note:** There should be an active [`tower_sessions`] middleware available
/// in the router's middleware stack.
pub fn build_router(authentication_service: AuthenticationService) -> Router {
    let state = State {
        authentication_service,
    };

    Router::new()
        .route(&LOGIN, get(routes::login))
        .route(&CALLBACK, get(routes::callback))
        .layer(Extension(state))
}
