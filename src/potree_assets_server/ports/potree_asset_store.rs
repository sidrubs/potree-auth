use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use async_trait::async_trait;

use crate::common::domain::static_asset::StaticAsset;

/// Defines the functionality needed to for the application to request static
/// `potree` assets.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PotreeAssetStore: Debug + Send + Sync + 'static {
    /// Read a specific `potree` asset by its path (e.g.
    /// "build/potree/potree.js")
    async fn get_asset(&self, path: &Path) -> Result<StaticAsset, PotreeAssetStoreError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum PotreeAssetStoreError {
    #[error("the asset ({path}) could not be found")]
    AssetNotFound { path: PathBuf },

    #[error("the asset ({path}) could not be parsed")]
    Parsing { path: PathBuf },
}
