use crate::authorization::domain::action::Action;
use crate::authorization::domain::error::AuthorizationEngineError;
use crate::authorization::domain::resource::ResourceType;
use crate::common::domain::User;
use crate::common::domain::value_objects::ProjectId;
use crate::common::ports::project_repository::ProjectRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum RenderingServiceError {
    #[error("project ({id}) not found")]
    ProjectNotFound { id: ProjectId },

    #[error("{} is not authorized to {} the {:?}: {}", user.name, action, resource_type, resource_name)]
    NotAuthorized {
        user: Box<User>,
        action: Box<Action>,
        resource_name: String,
        resource_type: Box<ResourceType>,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,

    #[error("the server is not configured correctly: {message}")]
    ServerConfiguration { message: String },

    #[error("{message}")]
    Infrastucture { message: String },
}

impl From<ProjectRepositoryError> for RenderingServiceError {
    fn from(value: ProjectRepositoryError) -> Self {
        match value {
            ProjectRepositoryError::ResourceNotFound { id }
            | ProjectRepositoryError::Parsing { id } => Self::ProjectNotFound { id },
            ProjectRepositoryError::Infrastucture { message } => Self::Infrastucture { message },
        }
    }
}

impl From<AuthorizationEngineError> for RenderingServiceError {
    fn from(value: AuthorizationEngineError) -> Self {
        match value {
            AuthorizationEngineError::NotAuthorized {
                user,
                action,
                resource_name,
                resource_type,
            } => Self::NotAuthorized {
                user,
                action,
                resource_name,
                resource_type,
            },
            AuthorizationEngineError::NotAuthenticated => Self::NotAuthenticated,
        }
    }
}

impl From<web_route::error::WebRouteError> for RenderingServiceError {
    fn from(value: web_route::error::WebRouteError) -> Self {
        Self::ServerConfiguration {
            message: format!("unable to build `WebRoute`: {value}"),
        }
    }
}

impl From<super::super::domain::error::RenderDomainError> for RenderingServiceError {
    fn from(value: super::super::domain::error::RenderDomainError) -> Self {
        match value {
            crate::render::domain::error::RenderDomainError::InvalidRoutePopulation { .. } => {
                Self::ServerConfiguration {
                    message: value.to_string(),
                }
            }
        }
    }
}
