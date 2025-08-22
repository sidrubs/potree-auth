use super::super::application::error::ProjectAssetsServiceError;
use crate::common::utils::http::api_error::ApiError;

impl From<ProjectAssetsServiceError> for ApiError {
    fn from(value: ProjectAssetsServiceError) -> Self {
        match value {
            ProjectAssetsServiceError::ProjectNotFound { id } => Self::ResourceNotFound {
                resource_name: format!("project: {id}"),
            },
            ProjectAssetsServiceError::NotAuthorized {
                user,
                action,
                resource_identifier,
                resource_type,
            } => Self::NotAuthorized {
                user,
                action,
                resource_identifier,
                resource_type,
            },
            ProjectAssetsServiceError::NotAuthenticated => Self::NotAuthenticated,
            ProjectAssetsServiceError::AssetNotFound { path } => Self::ResourceNotFound {
                resource_name: path.to_string_lossy().to_string(),
            },
            ProjectAssetsServiceError::Infrastucture { message } => Self::Infrastucture { message },
        }
    }
}
