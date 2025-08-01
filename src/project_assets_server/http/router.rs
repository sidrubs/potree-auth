use std::path::PathBuf;
use std::sync::LazyLock;

use axum::Extension;
use axum::Router;
use axum::routing::get;
use web_route::ParameterizedRoute;

use super::super::application::service::ProjectAssetService;
use super::routes;
use super::state::State;
use crate::common::domain::value_objects::ProjectId;

static ASSET_PATH: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/{project_id}/{*path}"));

#[derive(serde::Deserialize)]
pub(crate) struct AssetPathParams {
    pub project_id: ProjectId,
    pub path: PathBuf,
}

pub fn build_router(project_asset_service: ProjectAssetService) -> Router {
    let state = State {
        project_asset_service,
    };

    Router::new()
        .route(&ASSET_PATH, get(routes::project_asset))
        .layer(Extension(state))
}
