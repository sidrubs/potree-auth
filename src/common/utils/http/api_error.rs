use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;

use crate::authorization::domain::action::Action;
use crate::authorization::domain::resource::ResourceIdentifier;
use crate::authorization::domain::resource::ResourceType;
use crate::user::domain::User;

/// Errors that can be experienced by an API `axum` route handler.
///
/// All API route handlers should return this error. All domain specific errors
/// should be marshaled into an [`ApiError`] to ensure consistent HTTP
/// responses.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ApiError {
    #[error("unable to extract router state")]
    StateExtraction,

    #[error("unable to find resource: {resource_name}")]
    ResourceNotFound { resource_name: String },

    #[error("the resource already exists: {resource_name}")]
    ResourceAlreadyExists { resource_name: String },

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

    #[error("there is an issue with the the server infrastructure: {message}")]
    Infrastucture { message: String },

    #[error("there was a generic server error: {body}")]
    GenericServerError { body: Box<serde_json::Value> },
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
            ApiError::ResourceAlreadyExists { .. } => (StatusCode::CONFLICT).into_response(),
            ApiError::NotAuthorized { .. } => {
                (StatusCode::FORBIDDEN, self.to_string()).into_response()
            }
            ApiError::NotAuthenticated => (StatusCode::UNAUTHORIZED).into_response(),
            ApiError::GenericServerError { body } => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}
