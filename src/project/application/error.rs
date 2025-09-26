use crate::authorization::domain::action::Action;
use crate::authorization::domain::error::AuthorizationEngineError;
use crate::authorization::domain::resource::ResourceIdentifier;
use crate::authorization::domain::resource::ResourceType;
use crate::project::domain::ProjectId;
use crate::project::ports::project_repository::ProjectRepositoryError;
use crate::user::domain::User;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProjectServiceError {
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

    #[error("{message}")]
    Infrastucture { message: String },
}

impl From<ProjectRepositoryError> for ProjectServiceError {
    fn from(value: ProjectRepositoryError) -> Self {
        match value {
            ProjectRepositoryError::ResourceNotFound { id }
            | ProjectRepositoryError::Parsing { id } => Self::ProjectNotFound { id },
            ProjectRepositoryError::Infrastucture { message } => Self::Infrastucture { message },
        }
    }
}

impl From<AuthorizationEngineError> for ProjectServiceError {
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
