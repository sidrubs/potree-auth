use std::path::PathBuf;

use crate::potree_assets_server::ports::potree_asset_store::PotreeAssetStoreError;

#[derive(Debug, thiserror::Error)]
pub enum PotreeAssetsServiceError {
    #[error("the asset ({path}) could not be found")]
    AssetNotFound { path: PathBuf },
}

impl From<PotreeAssetStoreError> for PotreeAssetsServiceError {
    fn from(value: PotreeAssetStoreError) -> Self {
        match value {
            PotreeAssetStoreError::AssetNotFound { path }
            | PotreeAssetStoreError::Parsing { path } => Self::AssetNotFound { path },
        }
    }
}
