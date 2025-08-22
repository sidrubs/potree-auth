use super::super::application::error::PotreeAssetsServiceError;
use crate::common::utils::http::api_error::ApiError;

impl From<PotreeAssetsServiceError> for ApiError {
    fn from(value: PotreeAssetsServiceError) -> Self {
        match value {
            PotreeAssetsServiceError::AssetNotFound { path } => Self::ResourceNotFound {
                resource_name: path.to_string_lossy().to_string(),
            },
        }
    }
}
