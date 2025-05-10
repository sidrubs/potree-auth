mod health_check;
mod potree_asset;
mod potree_render;
mod project_asset;
pub(crate) mod state;

use std::{path::PathBuf, sync::Arc};

use axum::{Extension, Router, routing::get};
use state::ApplicationState;
use time::Duration;

use crate::services::{
    authorization::AuthorizationService, potree_assets::PotreeAssetService,
    project::ProjectService, project_assets::ProjectAssetService,
};

use super::middleware::session::apply_session_layer;

pub(crate) const HEALTH_CHECK: &str = "/_health";

pub(crate) const STATIC_POTREE: &str = "/static/potree";

pub(crate) const PROJECT_ROOT: &str = "/project";
pub(crate) const PROJECT_ASSETS: &str = "assets";

/// Axum route to reference a static `potree` asset.
fn potree_static_assets_route() -> PathBuf {
    PathBuf::new().join(STATIC_POTREE).join("{*path}")
}

/// Axum route to reference a specific project.
fn project_route() -> PathBuf {
    PathBuf::new().join(PROJECT_ROOT).join("{project_id}")
}

/// Axum route to reference a specific project asset.
fn project_asset() -> PathBuf {
    project_route().join(PROJECT_ASSETS).join("{*path}")
}

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
        .route(
            &potree_static_assets_route().to_string_lossy(),
            get(potree_asset::potree_asset),
        )
        .route(
            &project_asset().to_string_lossy(),
            get(project_asset::project_asset),
        )
        .route(
            &project_route().to_string_lossy(),
            get(potree_render::potree_render),
        )
        .layer(Extension(state));

    // Add web sessions
    let router = apply_session_layer(router, Duration::days(1));

    router
}
