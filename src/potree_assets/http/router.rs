use std::path::PathBuf;
use std::sync::LazyLock;

use axum::Extension;
use axum::Router;
use axum::routing::get;
use web_route::ParameterizedRoute;

use super::super::application::service::PotreeAssetService;
use super::routes;
use super::state::State;

static ASSET_PATH: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/{*path}"));

#[derive(serde::Deserialize)]
pub(crate) struct AssetPathParams {
    pub path: PathBuf,
}

pub fn build_router(potree_asset_service: PotreeAssetService) -> Router {
    let state = State {
        potree_asset_service,
    };

    Router::new()
        .route(&ASSET_PATH, get(routes::potree_asset))
        .layer(Extension(state))
}
