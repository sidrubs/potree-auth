use std::sync::LazyLock;

use axum::Extension;
use axum::Router;
use axum::routing::get;
use web_route::ParameterizedRoute;

use super::super::application::service::RenderingService;
use super::routes;
use super::state::State;
use crate::common::domain::value_objects::ProjectId;

static POTREE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/potree/{project_id}"));

#[derive(serde::Deserialize)]
pub(crate) struct PotreePathParams {
    pub project_id: ProjectId,
}

pub fn build_router(rendering_service: RenderingService) -> Router {
    let state = State {
        rendering_service: rendering_service,
    };

    Router::new()
        .route(&POTREE, get(routes::potree_render))
        .layer(Extension(state))
}
