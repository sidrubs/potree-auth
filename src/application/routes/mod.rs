mod health_check;
mod potree_asset;
mod project_asset;
pub(crate) mod state;

use std::sync::Arc;

use axum::{Extension, Router, routing::get};
use state::ApplicationState;

use crate::services::{
    authorization::AuthorizationService, potree_assets::PotreeAssetService,
    project::ProjectService, project_assets::ProjectAssetService,
};

const HEALTH_CHECK: &str = "/_health";
const POTREE_STATIC_ASSETS: &str = "/static/potree/{*path}";
const PROJECT_ASSETS: &str = "/project/{project_id}/assets/{*path}";

/// Initializes the application router, its state, and all of its routes.
pub fn build_router<AZ, P, POA, PRA>(
    authorization_service: AZ,
    project_service: P,
    project_asset_service: PRA,
    potree_asset_service: POA,
) -> Router
where
    AZ: AuthorizationService,
    P: ProjectService,
    POA: PotreeAssetService,
    PRA: ProjectAssetService,
{
    // Initialize application state.
    let state = ApplicationState {
        authorization_service: Arc::new(authorization_service),
        project_service: Arc::new(project_service),
        project_asset_service: Arc::new(project_asset_service),
        potree_asset_service: Arc::new(potree_asset_service),
    };

    // Build the router.
    let router = Router::new()
        .route(HEALTH_CHECK, get(health_check::health_check))
        .route(POTREE_STATIC_ASSETS, get(potree_asset::potree_asset))
        .route(PROJECT_ASSETS, get(project_asset::project_asset))
        .layer(Extension(state));

    router
}
