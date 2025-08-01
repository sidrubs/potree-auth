use crate::common::utils::axum::api_error::ApiError;
use crate::potree_assets_server::application::error::PotreeAssetsServiceError;

impl From<PotreeAssetsServiceError> for ApiError {
    fn from(value: PotreeAssetsServiceError) -> Self {
        match value {
            PotreeAssetsServiceError::AssetNotFound { path } => Self::ResourceNotFound {
                resource_name: path.to_string_lossy().to_string(),
            },
        }
    }
}
