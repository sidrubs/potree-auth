pub(crate) mod embedded;

use std::fmt::Debug;

use async_trait::async_trait;

use crate::domain::static_asset::StaticAsset;
use crate::error::ApplicationError;

/// Defines the functionality needed to for the application to request static
/// `potree` assets.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PotreeAssetStore: Debug + Send + Sync + 'static {
    /// Read a specific `potree` asset by its path (e.g.
    /// "build/potree/potree.js")
    async fn get_asset(&self, path: &str) -> Result<StaticAsset, ApplicationError>;
}
