use crate::common::domain::ResourceType;
use crate::common::domain::User;
use crate::common::domain::value_objects::ProjectId;
use crate::common::ports::authorization_engine::AuthorizationEngineError;
use crate::common::ports::project_datastore::ProjectDatastoreError;

#[derive(Debug, thiserror::Error)]
pub enum RenderingServiceError {
    #[error("project ({id}) not found")]
    ProjectNotFound { id: ProjectId },

    #[error("{} is not authorized to view the {:?}: {}", user.name, resource_type, resource_name)]
    NotAuthorized {
        user: Box<User>,
        resource_name: String,
        resource_type: Box<ResourceType>,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,

    #[error("the server is not configured correctly: {message}")]
    ServerConfiguration { message: String },
}

impl From<ProjectDatastoreError> for RenderingServiceError {
    fn from(value: ProjectDatastoreError) -> Self {
        match value {
            ProjectDatastoreError::ResourceNotFound { id }
            | ProjectDatastoreError::Parsing { id } => Self::ProjectNotFound { id },
        }
    }
}

impl From<AuthorizationEngineError> for RenderingServiceError {
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

impl From<web_route::error::WebRouteError> for RenderingServiceError {
    fn from(value: web_route::error::WebRouteError) -> Self {
        Self::ServerConfiguration {
            message: format!("unable to build `WebRoute`: {value}"),
        }
    }
}
