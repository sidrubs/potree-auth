use std::path::PathBuf;

use super::super::ports::project_asset_store::ProjectAssetStoreError;
use crate::authorization::domain::action::Action;
use crate::authorization::domain::error::AuthorizationEngineError;
use crate::authorization::domain::resource::ResourceIdentifier;
use crate::authorization::domain::resource::ResourceType;
use crate::project::application::error::ProjectServiceError;
use crate::project::domain::ProjectId;
use crate::user::domain::User;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProjectAssetsServiceError {
    #[error("project ({id}) not found")]
    ProjectNotFound { id: ProjectId },

    #[error("{} is not authorized to {} the {:?}: {:?}", user.name, action, resource_type, resource_identifier)]
    NotAuthorized {
        user: Box<User>,
        action: Action,
        resource_identifier: Option<ResourceIdentifier>,
        resource_type: ResourceType,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,

    #[error("the asset ({path}) could not be found")]
    AssetNotFound { path: PathBuf },

    #[error("{message}")]
    Infrastucture { message: String },
}

impl From<ProjectServiceError> for ProjectAssetsServiceError {
    fn from(value: ProjectServiceError) -> Self {
        match value {
            ProjectServiceError::ProjectNotFound { id } => Self::ProjectNotFound { id },
            ProjectServiceError::NotAuthorized {
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
            ProjectServiceError::NotAuthenticated => Self::NotAuthenticated,
            ProjectServiceError::Infrastucture { message } => Self::Infrastucture { message },
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
                action,
                resource_identifier,
                resource_type,
            } => Self::NotAuthorized {
                user,
                action,
                resource_identifier,
                resource_type,
            },
            AuthorizationEngineError::NotAuthenticated => Self::NotAuthenticated,
        }
    }
}
