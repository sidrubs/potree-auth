use std::sync::Arc;
use std::sync::LazyLock;

use axum::Router;
use time::Duration;
use tower::Layer;
use tower_http::normalize_path::NormalizePath;
use tower_http::normalize_path::NormalizePathLayer;
use web_route::ParameterizedRoute;
use web_route::WebRoute;

use super::super::config::PotreeAuthConfiguration;
use super::super::error::PotreeAuthError;
use super::factories::init_authentication_engine;
use super::factories::init_authorization_engine;
use super::middleware::session::apply_session_layer;
use super::middleware::tracing::apply_tracing_middleware;
use crate::authentication::application::service::AuthenticationService;
use crate::authentication::http::LOGIN;
use crate::authentication::{self};
use crate::common;
use crate::common::adapters::project_datastore::manifest_file::ManifestFileProjectDatastore;
use crate::potree_assets::adapters::potree_asset_store::embedded::EmbeddedPotreeAssetStore;
use crate::potree_assets::application::service::PotreeAssetService;
use crate::potree_assets::{self};
use crate::project_assets::adapters::project_asset_store::serve_dir::ServeDirProjectAssets;
use crate::project_assets::application::service::ProjectAssetService;
use crate::project_assets::http::ASSET_PATH;
use crate::project_assets::{self};
use crate::render::application::service::RenderingService;
use crate::render::{self};

pub static AUTH: LazyLock<WebRoute> = LazyLock::new(|| WebRoute::new("/auth"));
pub static POTREE_ASSETS: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/potree-assets"));
pub static PROJECT_ASSETS: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/project-assets"));

pub async fn init_application(
    config: PotreeAuthConfiguration,
) -> Result<NormalizePath<Router>, PotreeAuthError> {
    // Initialize adaptors
    let authorization_engine = init_authorization_engine(config.idp.is_some());
    let authentication_engine = init_authentication_engine(config.idp).await?;
    let project_datastore = Arc::new(ManifestFileProjectDatastore::new(&config.data_dir));
    let potree_asset_store = Arc::new(EmbeddedPotreeAssetStore);
    let project_asset_store = Arc::new(ServeDirProjectAssets::new(&config.data_dir));

    // Intialize services
    let authentication_service = AuthenticationService::new(authentication_engine);
    let potree_asset_service = PotreeAssetService::new(potree_asset_store);
    let project_asset_service = ProjectAssetService::new(
        project_datastore.clone(),
        project_asset_store,
        authorization_engine.clone(),
    );
    let rendering_service = RenderingService::new(
        project_datastore,
        authorization_engine,
        PROJECT_ASSETS.join(ASSET_PATH.as_ref()),
        WebRoute::new(POTREE_ASSETS.as_ref()),
    );

    Ok(build_router(
        authentication_service,
        potree_asset_service,
        project_asset_service,
        rendering_service,
    ))
}

/// Sets up the http router with its various services.
fn build_router(
    authentication_service: AuthenticationService,
    potree_asset_service: PotreeAssetService,
    project_asset_service: ProjectAssetService,
    rendering_service: RenderingService,
) -> NormalizePath<Router> {
    // Initialize child routers
    let authentication_router = authentication::http::build_router(authentication_service);
    let potree_asset_router = potree_assets::http::build_router(potree_asset_service);
    let project_asset_router = project_assets::http::build_router(project_asset_service);
    let rendering_router = render::http::build_router(rendering_service, AUTH.join(LOGIN.as_ref()));
    let common_routes = common::utils::axum::common_routes::build_router();

    // Build top-level router
    let router = Router::new()
        .nest(&AUTH, authentication_router)
        .nest(&POTREE_ASSETS, potree_asset_router)
        .nest(&PROJECT_ASSETS, project_asset_router)
        .merge(rendering_router)
        .merge(common_routes);

    // Apply middleware
    let router = apply_session_layer(router, Duration::days(1));
    let router = apply_tracing_middleware(router);

    NormalizePathLayer::trim_trailing_slash().layer(router)
}
