mod auth;
mod health_check;
mod potree_asset;
mod potree_render;
mod project_asset;
pub(crate) mod state;

use std::{path::Path, sync::Arc};

use axum::{Extension, Router, routing::get};
use path_slash::PathExt;
use state::ApplicationState;
use time::Duration;

use crate::services::{
    authentication::AuthenticationService, authorization::AuthorizationService,
    potree_assets::PotreeAssetService, project::ProjectService,
    project_assets::ProjectAssetService,
};

use super::middleware::{session::apply_session_layer, tracing::apply_tracing_middleware};

pub(crate) const HEALTH_CHECK: &str = "/_health";

pub(crate) const STATIC_POTREE: &str = "/static/potree";

pub(crate) const PROJECT_ROOT: &str = "/project";
pub(crate) const PROJECT_ASSETS: &str = "assets";

pub(crate) const AUTH_ROOT: &str = "/auth";
pub(crate) const AUTH_LOGIN: &str = "login";
pub(crate) const AUTH_CALLBACK: &str = "callback";

/// Axum route to reference a static `potree` asset.
fn potree_static_assets_route() -> String {
    Path::new(STATIC_POTREE)
        .join("{*path}")
        .to_slash_lossy()
        .to_string()
}

/// Axum route to reference a specific project.
fn project_route() -> String {
    Path::new(PROJECT_ROOT)
        .join("{project_id}")
        .to_slash_lossy()
        .to_string()
}

/// Axum route to reference a specific project asset.
fn project_asset() -> String {
    Path::new(&project_route())
        .join(PROJECT_ASSETS)
        .join("{*path}")
        .to_slash_lossy()
        .to_string()
}

/// Route to initialize an OIDC login of the application.
fn login_route() -> String {
    Path::new(AUTH_ROOT)
        .join(AUTH_LOGIN)
        .to_slash_lossy()
        .to_string()
}

/// Route to finalize an OIDC login of the application.
fn callback_route() -> String {
    Path::new(AUTH_ROOT)
        .join(AUTH_CALLBACK)
        .to_slash_lossy()
        .to_string()
}

/// Initializes the application router, its state, and all of its routes.
pub fn build_router<AZ, AN, P, POA, PRA>(
    authorization_service: AZ,
    authentication_service: AN,
    project_service: P,
    project_asset_service: PRA,
    potree_asset_service: POA,
) -> Router
where
    AZ: AuthorizationService,
    AN: AuthenticationService,
    P: ProjectService,
    POA: PotreeAssetService,
    PRA: ProjectAssetService,
{
    // Initialize application state.
    let state = ApplicationState {
        authorization_service: Arc::new(authorization_service),
        authentication_service: Arc::new(authentication_service),
        project_service: Arc::new(project_service),
        project_asset_service: Arc::new(project_asset_service),
        potree_asset_service: Arc::new(potree_asset_service),
    };

    // Build the router.
    let router = Router::new()
        .route(HEALTH_CHECK, get(health_check::health_check))
        .route(
            &potree_static_assets_route(),
            get(potree_asset::potree_asset),
        )
        .route(&project_asset(), get(project_asset::project_asset))
        .route(&project_route(), get(potree_render::potree_render))
        .layer(Extension(state));

    // Apply middleware
    let router = apply_session_layer(router, Duration::days(1));
    let router = apply_tracing_middleware(router);

    router
}
