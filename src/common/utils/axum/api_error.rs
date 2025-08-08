use axum::response::IntoResponse;
use http::StatusCode;

use crate::common::domain::ResourceType;
use crate::common::domain::User;

/// Errors that can be experienced by an API `axum` route handler.
///
/// All API route handlers should return this error. All domain specific errors
/// should be marshalled into an [`ApiError`] to ensure consistent HTTP
/// responses.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ApiError {
    #[error("unable to extract router state")]
    StateExtraction,

    #[error("unable to find resource: {resource_name}")]
    ResourceNotFound { resource_name: String },

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

    #[error("there is an issue with the the server infrastructure: {message}")]
    Infrastucture { message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // Log the error.
        tracing::error!("{self}");

        // Generate a response to send to the client.
        match self {
            ApiError::StateExtraction
            | ApiError::ServerConfiguration { .. }
            | ApiError::Infrastucture { .. } => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            ApiError::ResourceNotFound { .. } => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            ApiError::NotAuthorized { .. } => {
                (StatusCode::FORBIDDEN, self.to_string()).into_response()
            }
            ApiError::NotAuthenticated => (StatusCode::UNAUTHORIZED).into_response(),
        }
    }
}
