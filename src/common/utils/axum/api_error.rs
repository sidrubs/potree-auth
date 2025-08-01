use axum::response::IntoResponse;

use crate::common::domain::ResourceType;
use crate::common::domain::User;

/// Errors that can be experienced by an API `axum` router.
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

    #[error("unable to handle type conversion: {resource_name}")]
    TypeConversion { resource_name: String },

    #[error("{} is not authorized to view the {:?}: {}", user.name, resource_type, resource_name)]
    NotAuthorized {
        user: User,
        resource_name: String,
        resource_type: ResourceType,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,

    #[error("the server is not configured correctly: {message}")]
    ServerConfiguration { message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
