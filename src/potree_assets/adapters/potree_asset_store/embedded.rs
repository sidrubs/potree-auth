use std::path::Path;

use async_trait::async_trait;
use potree_embed::PotreeAssets;

use super::super::super::ports::potree_asset_store::PotreeAssetStore;
use super::super::super::ports::potree_asset_store::PotreeAssetStoreError;
use crate::common::domain::StaticAsset;

/// Provides access to built `potree` static assets that are embedded in the
/// Rust binary.
#[derive(Debug, Clone)]
pub(crate) struct EmbeddedPotreeAssetStore;

impl EmbeddedPotreeAssetStore {
    #[tracing::instrument]
    fn get_asset(&self, path: &Path) -> Result<StaticAsset, PotreeAssetStoreError> {
        let embedded_asset = PotreeAssets::get(&path.to_string_lossy()).ok_or(
            PotreeAssetStoreError::AssetNotFound {
                path: path.to_owned(),
            },
        )?;

        StaticAsset::from_rust_embed(embedded_asset, path).map_err(|_e| {
            PotreeAssetStoreError::Parsing {
                path: path.to_owned(),
            }
        })
    }
}

#[async_trait]
impl PotreeAssetStore for EmbeddedPotreeAssetStore {
    async fn get_asset(&self, path: &Path) -> Result<StaticAsset, PotreeAssetStoreError> {
        Self::get_asset(self, path)
    }
}

#[cfg(test)]
mod embedded_potree_asset_service_tests {
    use super::*;

    mod get_asset {
        use http::header;

        use super::*;

        #[test]
        fn should_return_a_valid_asset_if_it_exists() {
            // Arrange
            let asset_service = EmbeddedPotreeAssetStore;

            // Act
            let static_asset = asset_service
                .get_asset(&Path::new("build/potree/potree.js"))
                .expect("asset should exist");

            // Assert
            assert_eq!(
                static_asset.0.headers().get(header::CONTENT_TYPE).unwrap(),
                mime::TEXT_JAVASCRIPT.as_ref()
            );
        }

        #[test]
        fn should_return_correct_error_if_asset_does_not_exist() {
            // Arrange
            let non_existent_path = Path::new("build/non/existent.txt");
            let asset_service = EmbeddedPotreeAssetStore;

            // Act
            let res = asset_service.get_asset(non_existent_path);

            // Assert
            assert!(
                matches!(res, Err(PotreeAssetStoreError::AssetNotFound{ path }) if path == non_existent_path)
            );
        }
    }
}
