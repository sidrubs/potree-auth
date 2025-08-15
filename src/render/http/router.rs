use std::sync::LazyLock;

use axum::Extension;
use axum::Router;
use axum::routing::get;
use web_route::ParameterizedRoute;
use web_route::WebRoute;

use super::super::application::service::RenderingService;
use super::route_handlers;
use super::state::State;
use crate::common::domain::value_objects::ProjectId;

pub static POTREE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/potree/{project_id}"));
pub static PROJECT_DASHBOARD: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/projects"));

#[derive(serde::Deserialize)]
pub(crate) struct PotreePathParams {
    pub project_id: ProjectId,
}

/// Builds a routes for rendering HTML pages.
///
/// `login_route` defines where the user should be redirected if they need to be
/// authenticated.
pub fn build_router(rendering_service: RenderingService, login_route: WebRoute) -> Router {
    let state = State {
        rendering_service,
        login_route,
    };

    Router::new()
        .route(&POTREE, get(route_handlers::potree_render))
        .route(&PROJECT_DASHBOARD, get(route_handlers::project_dashboard))
        .layer(Extension(state))
}
