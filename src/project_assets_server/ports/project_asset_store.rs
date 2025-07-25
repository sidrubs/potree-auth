use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use async_trait::async_trait;
use http::HeaderMap;

use crate::common::domain::static_asset::StaticAsset;

/// Defines the functionality needed to for an application to request static
/// project assets.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProjectAssetStore: Debug + Send + Sync + 'static {
    /// Read a specific project asset by its path and request headers as the
    /// headers provide various instructions as to how to format the data.
    async fn get_asset(
        &self,
        path: &Path,
        request_headers: Option<HeaderMap>,
    ) -> Result<StaticAsset, ProjectAssetStoreError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProjectAssetStoreError {
    #[error("the asset ({path}) could not be found")]
    AssetNotFound { path: PathBuf },

    #[error("the asset ({path}) could not be parsed")]
    Parsing { path: PathBuf },
}
