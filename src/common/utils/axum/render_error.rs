use axum::response::IntoResponse;

use crate::common::domain::ResourceType;
use crate::common::domain::User;

/// Errors that can be experienced by a rendering (e.g. HTML) `axum` route
/// handler.
///
/// All rendering route handlers should return this error. All domain specific
/// errors should be marshalled into an [`RenderError`] to ensure consistent
/// HTTP responses.
///
/// > A [`RenderError`] is different to an
/// > [`ApiError`][`super::api_error::ApiError`] in that it responds with a user
/// > facing error (i.e. HTML).
#[derive(Debug, Clone, thiserror::Error)]
pub enum RenderError {
    #[error("unable to extract router state")]
    StateExtraction,

    #[error("unable to find resource: {resource_name}")]
    ResourceNotFound { resource_name: String },

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

    #[error("a generic server error: {message}")]
    ServerError { message: String },

    #[error("an error was experienced during authentication: {message}")]
    AuthenticationFlow { message: String },
}

impl IntoResponse for RenderError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
