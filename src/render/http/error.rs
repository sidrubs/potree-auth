use super::super::application::error::RenderingServiceError;
use crate::common::utils::axum::render_error::RenderError;

impl From<RenderingServiceError> for RenderError {
    fn from(value: RenderingServiceError) -> Self {
        match value {
            RenderingServiceError::ProjectNotFound { id } => Self::ResourceNotFound {
                resource_name: format!("project: {id}"),
            },
            RenderingServiceError::NotAuthorized {
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
            RenderingServiceError::NotAuthenticated => Self::NotAuthenticated,
            RenderingServiceError::ServerConfiguration { message } => {
                Self::ServerConfiguration { message }
            }
            RenderingServiceError::Infrastucture { message } => Self::Infrastucture { message },
        }
    }
}

impl From<askama::Error> for RenderError {
    fn from(value: askama::Error) -> Self {
        Self::ServerError {
            message: format!("error rendering template: {value}"),
        }
    }
}
