use std::sync::LazyLock;

use axum::Extension;
use axum::Router;
use axum::routing::get;
use web_route::ParameterizedRoute;

use crate::potree_assets_server::application::service::PotreeAssetService;

use super::routes;
use super::state::State;

static ASSET_PATH: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/{*path}"));

pub fn build_router(potree_asset_service: PotreeAssetService) -> Router {
    let state = State {
        potree_asset_service,
    };

    Router::new()
        .route(&ASSET_PATH, get(routes::potree_asset))
        .layer(Extension(state))
}
