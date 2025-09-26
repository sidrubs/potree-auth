use crate::authorization::domain::action::Action;
use crate::authorization::domain::error::AuthorizationEngineError;
use crate::authorization::domain::resource::ResourceIdentifier;
use crate::authorization::domain::resource::ResourceType;
use crate::project::application::error::ProjectServiceError;
use crate::project::domain::ProjectId;
use crate::user::domain::User;

#[derive(Debug, thiserror::Error)]
pub enum RenderingServiceError {
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

    #[error("the server is not configured correctly: {message}")]
    ServerConfiguration { message: String },

    #[error("{message}")]
    Infrastucture { message: String },
}

impl From<ProjectServiceError> for RenderingServiceError {
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

impl From<AuthorizationEngineError> for RenderingServiceError {
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
