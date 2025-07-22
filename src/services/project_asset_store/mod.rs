pub(crate) mod serve_dir;

use async_trait::async_trait;
use http::HeaderMap;
use std::fmt::Debug;

use crate::{domain::static_asset::StaticAsset, error::ApplicationError};

/// Defines the functionality needed to for the application to request static
/// project assets.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProjectAssetStore: Debug + Send + Sync + 'static {
    /// Read a specific project asset by its path and request headers as the
    /// headers provide various instructions as to how to format the data.
    ///
    /// # Errors
    ///
    /// Should return an error if the path contains a parent directory
    /// reference. This is for illegal path traversal purposes.
    async fn get_asset(
        &self,
        path: &str,
        request_headers: Option<HeaderMap>,
    ) -> Result<StaticAsset, ApplicationError>;
}
