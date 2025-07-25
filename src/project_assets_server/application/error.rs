use super::super::ports::project_asset_store::ProjectAssetStoreError;
use super::super::ports::project_datastore::ProjectDatastoreError;
use crate::common::domain::User;
use crate::common::domain::value_objects::ProjectId;
use crate::common::ports::authorization_engine::AuthorizationEngineError;
use crate::common::ports::authorization_engine::ResourceType;

#[derive(Debug, thiserror::Error)]
pub enum ProjectServiceError {
    #[error("project ({id}) not found")]
    ProjectNotFound { id: ProjectId },

    #[error("{} is not authorized to view the {:?}: {}", user.name, resource_type, resource_name)]
    NotAuthorized {
        user: User,
        resource_name: String,
        resource_type: ResourceType,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,

    #[error("{0}")]
    ProjectAsset(String),
}

impl From<ProjectDatastoreError> for ProjectServiceError {
    fn from(value: ProjectDatastoreError) -> Self {
        match value {
            ProjectDatastoreError::ResourceNotFound { id }
            | ProjectDatastoreError::Parsing { id } => Self::ProjectNotFound { id },
        }
    }
}

impl From<ProjectAssetStoreError> for ProjectServiceError {
    fn from(value: ProjectAssetStoreError) -> Self {
        Self::ProjectAsset(value.to_string())
    }
}

impl From<AuthorizationEngineError> for ProjectServiceError {
    fn from(value: AuthorizationEngineError) -> Self {
        match value {
            AuthorizationEngineError::NotAuthorized {
                user,
                resource_name,
                resource_type,
            } => Self::NotAuthorized {
                user,
                resource_name,
                resource_type,
            },
            AuthorizationEngineError::NotAuthenticated => Self::NotAuthenticated,
        }
    }
}
