use std::path::PathBuf;

use super::super::ports::project_asset_store::ProjectAssetStoreError;
use super::super::ports::project_datastore::ProjectDatastoreError;
use crate::common::domain::User;
use crate::common::domain::value_objects::ProjectId;
use crate::common::ports::authorization_engine::AuthorizationEngineError;
use crate::common::ports::authorization_engine::ResourceType;

#[derive(Debug, thiserror::Error)]
pub enum ProjectAssetsServiceError {
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

    #[error("the asset ({path}) could not be found")]
    AssetNotFound { path: PathBuf },
}

impl From<ProjectDatastoreError> for ProjectAssetsServiceError {
    fn from(value: ProjectDatastoreError) -> Self {
        match value {
            ProjectDatastoreError::ResourceNotFound { id }
            | ProjectDatastoreError::Parsing { id } => Self::ProjectNotFound { id },
        }
    }
}

impl From<ProjectAssetStoreError> for ProjectAssetsServiceError {
    fn from(value: ProjectAssetStoreError) -> Self {
        match value {
            ProjectAssetStoreError::AssetNotFound { path }
            | ProjectAssetStoreError::Parsing { path } => Self::AssetNotFound { path },
        }
    }
}

impl From<AuthorizationEngineError> for ProjectAssetsServiceError {
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
