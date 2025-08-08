use axum::response::IntoResponse;
use http::StatusCode;

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
        user: Box<User>,
        resource_name: String,
        resource_type: Box<ResourceType>,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,

    #[error("the server is not configured correctly: {message}")]
    ServerConfiguration { message: String },

    #[error("a generic server error: {message}")]
    ServerError { message: String },

    #[error("an error was experienced during authentication: {message}")]
    AuthenticationFlow { message: String },

    #[error("there is an issue with the the server infrastructure: {message}")]
    Infrastucture { message: String },
}

/// This should really be updated to redirect the user to an error page.
impl IntoResponse for RenderError {
    fn into_response(self) -> axum::response::Response {
        // Log the error.
        tracing::error!("{self}");

        // Generate a response to send to the client.
        match self {
            RenderError::StateExtraction
            | RenderError::ServerConfiguration { .. }
            | RenderError::ServerError { .. }
            | RenderError::Infrastucture { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            RenderError::ResourceNotFound { .. } => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            RenderError::NotAuthorized { .. } => {
                (StatusCode::FORBIDDEN, self.to_string()).into_response()
            }
            RenderError::NotAuthenticated | RenderError::AuthenticationFlow { .. } => {
                (StatusCode::UNAUTHORIZED).into_response()
            }
        }
    }
}
