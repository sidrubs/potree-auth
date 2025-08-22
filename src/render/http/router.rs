use std::sync::LazyLock;

use axum::Extension;
use axum::Router;
use axum::routing::get;
use web_route::ParameterizedRoute;
use web_route::WebRoute;

use super::super::application::service::RenderingService;
use super::route_handlers;
use super::state::State;
use crate::common::utils::axum::render_error::RenderError;
use crate::project::domain::ProjectId;
use crate::render::http::middleware::potree_csp::set_potree_csp;

pub static POTREE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/potree/{project_id}"));
pub static PROJECT_DASHBOARD: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/projects"));
pub static NOT_FOUND: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/404"));

#[derive(serde::Deserialize)]
pub(crate) struct PotreePathParams {
    pub project_id: ProjectId,
}

/// Builds a routes for rendering HTML pages.
///
/// `login_route` defines where the user should be redirected if they need to be
/// authenticated.
pub fn build_router(
    rendering_service: RenderingService,
    login_route: WebRoute,
) -> Result<Router, RenderError> {
    let state = State {
        rendering_service,
        login_route,
    };

    let router = Router::new()
        .route(
            &POTREE,
            get(route_handlers::potree_render).layer(set_potree_csp()?),
        )
        .route(&PROJECT_DASHBOARD, get(route_handlers::project_dashboard))
        .route(&NOT_FOUND, get(route_handlers::not_found))
        .fallback(get(route_handlers::not_found))
        .layer(Extension(state));

    Ok(router)
}
