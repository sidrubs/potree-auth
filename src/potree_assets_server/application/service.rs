use std::path::Path;
use std::sync::Arc;

use crate::common::domain::StaticAsset;
use crate::potree_assets_server::application::error::PotreeAssetsServiceError;
use crate::potree_assets_server::ports::potree_asset_store::PotreeAssetStore;

/// A service for loading potree assets.
pub struct PotreeAssetService {
    potree_asset_store: Arc<dyn PotreeAssetStore>,
}

impl PotreeAssetService {
    pub fn new(potree_asset_store: Arc<dyn PotreeAssetStore>) -> Self {
        Self { potree_asset_store }
    }

    /// Read a specific potree asset.
    pub async fn request_asset(
        &self,
        asset_path: &Path,
    ) -> Result<StaticAsset, PotreeAssetsServiceError> {
        Ok(self.potree_asset_store.get_asset(&asset_path).await?)
    }
}
