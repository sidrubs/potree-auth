use std::sync::Arc;

use crate::services::{
    authorization::AuthorizationService, potree_assets::PotreeAssetService, project::ProjectService,
};

/// The state that is available throughout the application router.
#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub authorization_service: Arc<dyn AuthorizationService>,
    pub project_service: Arc<dyn ProjectService>,
    pub potree_asset_service: Arc<dyn PotreeAssetService>,
}
