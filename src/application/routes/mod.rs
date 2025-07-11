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
use tower_http::set_header::SetResponseHeaderLayer;
use web_route::{ParameterizedRoute, WebRoute};

use crate::{
    domain::value_objects::ProjectId,
    services::{
        authentication::AuthenticationService, authorization::AuthorizationService,
        potree_assets::PotreeAssetService, project::ProjectService,
        project_assets::ProjectAssetService,
    },
};

use super::middleware::{session::apply_session_layer, tracing::apply_tracing_middleware};

pub(crate) static HEALTH_CHECK: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/_health"));

pub(crate) static STATIC_POTREE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/static/potree"));
pub(crate) static STATIC_POTREE_ASSET_ROUTE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| STATIC_POTREE.join("{*asset_path}"));

pub(crate) static PROJECT_ROOT: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/project"));
pub(crate) static PROJECT_ROUTE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| PROJECT_ROOT.join("/{project_id}"));
pub(crate) static PROJECT_ASSET: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| PROJECT_ROUTE.join("/assets/{*path}"));

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
    authorization_service: Arc<dyn AuthorizationService>,
    authentication_service: Arc<dyn AuthenticationService>,
    project_service: Arc<dyn ProjectService>,
    project_asset_service: Arc<dyn ProjectAssetService>,
    potree_asset_service: Arc<dyn PotreeAssetService>,
) -> Router {
    // Initialize application state.
    let state = ApplicationState {
        authorization_service,
        authentication_service,
        project_service,
        project_asset_service,
        potree_asset_service,
    };

    // Middleware to add cache control in an attempt to stop CDNs from caching the data.
    let add_cache_control = SetResponseHeaderLayer::overriding(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );

    // Build the router.
    let router = Router::new()
        .route(&HEALTH_CHECK, get(health_check::health_check))
        .route(&STATIC_POTREE_ASSET_ROUTE, get(potree_asset::potree_asset))
        .route(
            &PROJECT_ASSET,
            get(project_asset::project_asset).layer(add_cache_control),
        )
        .route(&PROJECT_ROUTE, get(potree_render::potree_render))
        .route(&AUTH_LOGIN, get(auth::login))
        .route(&AUTH_CALLBACK, get(auth::callback))
        .layer(Extension(state));

    // Apply middleware
    let router = apply_session_layer(router, Duration::days(1));
    apply_tracing_middleware(router)
}
