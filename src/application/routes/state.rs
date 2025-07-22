use std::sync::Arc;

use crate::services::authentication_service::AuthenticationService;
use crate::services::authorization_engine::AuthorizationEngine;
use crate::services::potree_asset_store::PotreeAssetStore;
use crate::services::project_asset_store::ProjectAssetStore;
use crate::services::project_store::ProjectService;

/// The state that is available throughout the application router.
#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub authorization_service: Arc<dyn AuthorizationEngine>,
    pub authentication_service: Arc<dyn AuthenticationService>,
    pub project_service: Arc<dyn ProjectService>,
    pub project_asset_service: Arc<dyn ProjectAssetStore>,
    pub potree_asset_service: Arc<dyn PotreeAssetStore>,
}
