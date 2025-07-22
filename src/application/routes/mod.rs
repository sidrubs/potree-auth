mod auth;
mod health_check;
mod potree_asset;
mod potree_render;
mod project_asset;
pub(crate) mod state;

use std::sync::{Arc, LazyLock};

use axum::{Extension, Router, routing::get};
use http::{HeaderValue, header};
use state::ApplicationState;
use time::Duration;
use tower::Layer;
use tower_http::{
    normalize_path::{NormalizePath, NormalizePathLayer},
    set_header::SetResponseHeaderLayer,
};
use web_route::{ParameterizedRoute, WebRoute};

use crate::{
    domain::value_objects::ProjectId,
    services::{
        authentication_service::AuthenticationService, authorization_engine::AuthorizationEngine,
        potree_asset_store::PotreeAssetStore, project_asset_store::ProjectAssetStore,
        project_store::ProjectService,
    },
};

use super::middleware::{session::apply_session_layer, tracing::apply_tracing_middleware};

pub(crate) static HEALTH_CHECK: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/_health"));

pub(crate) static POTREE_ASSETS_ROOT: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/potree-assets"));
pub(crate) static POTREE_ASSETS: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| POTREE_ASSETS_ROOT.join("/{*path}"));
pub(crate) static PROJECT_ASSETS_ROOT: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/project-assets"));
pub(crate) static PROJECT_ASSETS: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| PROJECT_ASSETS_ROOT.join("/{project_id}/{*path}"));

pub(crate) static POTREE_UI: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/potree"));
pub(crate) static POTREE_UI_PROJECT: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| POTREE_UI.join("/{project_id}"));

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ProjectAssetParams {
    pub(crate) project_id: ProjectId,
    pub(crate) path: WebRoute,
}

pub(crate) static AUTH_ROOT: LazyLock<WebRoute> = LazyLock::new(|| WebRoute::new("/auth"));
pub(crate) static AUTH_LOGIN: LazyLock<WebRoute> = LazyLock::new(|| AUTH_ROOT.join("/login"));
pub(crate) static AUTH_CALLBACK: LazyLock<WebRoute> = LazyLock::new(|| AUTH_ROOT.join("/callback"));

/// Initializes the application router, its state, and all of its routes.
pub fn build_router(
    authorization_service: Arc<dyn AuthorizationEngine>,
    authentication_service: Arc<dyn AuthenticationService>,
    project_service: Arc<dyn ProjectService>,
    project_asset_service: Arc<dyn ProjectAssetStore>,
    potree_asset_service: Arc<dyn PotreeAssetStore>,
) -> NormalizePath<Router> {
    // Initialize application state.
    let state = ApplicationState {
        authorization_service,
        authentication_service,
        project_service,
        project_asset_service,
        potree_asset_service,
    };

    // Middleware to add cache control in an attempt to stop CDNs from caching the
    // data.
    let add_cache_control = SetResponseHeaderLayer::overriding(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );

    // Build the router.
    let router = Router::new()
        .route(&HEALTH_CHECK, get(health_check::health_check))
        .route(&POTREE_ASSETS, get(potree_asset::potree_asset))
        .route(
            &PROJECT_ASSETS,
            get(project_asset::project_asset).layer(add_cache_control),
        )
        .route(&POTREE_UI_PROJECT, get(potree_render::potree_render))
        .route(&AUTH_LOGIN, get(auth::login))
        .route(&AUTH_CALLBACK, get(auth::callback))
        .layer(Extension(state));

    // Apply middleware
    let router = apply_session_layer(router, Duration::days(1));
    let router = apply_tracing_middleware(router);

    NormalizePathLayer::trim_trailing_slash().layer(router)
}
