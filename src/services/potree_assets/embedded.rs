use async_trait::async_trait;
use potree_embed::PotreeAssets;

use crate::{
    domain::{ResourceType, StaticAsset},
    error::ApplicationError,
};

use super::PotreeAssetService;

/// Provides access to built `potree` static assets that are embedded in the
/// Rust binary.
#[derive(Debug, Clone)]
pub(crate) struct EmbeddedPotreeAssetService;

impl EmbeddedPotreeAssetService {
    #[tracing::instrument]
    fn get_asset(&self, path: &str) -> Result<StaticAsset, ApplicationError> {
        let embedded_asset = PotreeAssets::get(path).ok_or(ApplicationError::ResourceNotFound {
            resource_name: path.to_owned(),
            resource_type: ResourceType::StaticAsset,
        })?;

        Ok(StaticAsset::from_rust_embed(embedded_asset, path)?)
    }
}

#[async_trait]
impl PotreeAssetService for EmbeddedPotreeAssetService {
    async fn get_asset(&self, path: &str) -> Result<StaticAsset, ApplicationError> {
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
            let asset_service = EmbeddedPotreeAssetService;

            // Act
            let static_asset = asset_service
                .get_asset("build/potree/potree.js")
                .expect("unable to find asset");

            // Assert
            assert_eq!(
                static_asset.0.headers().get(header::CONTENT_TYPE).unwrap(),
                mime::TEXT_JAVASCRIPT.as_ref()
            )
        }

        #[test]
        fn should_return_correct_error_if_asset_does_not_exist() {
            // Arrange
            let non_existent_path = "build/non/existent.txt";
            let asset_service = EmbeddedPotreeAssetService;

            // Act
            let res = asset_service.get_asset(non_existent_path);

            // Assert
            assert!(
                matches!(res, Err(ApplicationError::ResourceNotFound { resource_name, resource_type }) if resource_name == non_existent_path && resource_type == ResourceType::StaticAsset    )
            )
        }
    }
}
