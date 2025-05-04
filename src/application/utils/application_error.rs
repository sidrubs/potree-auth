use axum::response::{IntoResponse, Response};
use http::StatusCode;

use crate::error::ApplicationError;

/// Defines how `axum` should convert [`ApplicationError`]'s into an HTTP
/// response.
impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match &self {
            ApplicationError::NotAuthorized { .. } => (StatusCode::UNAUTHORIZED, self.to_string()),
            ApplicationError::ResourceNotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            ApplicationError::ServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.to_owned())
            }
            ApplicationError::StateExtraction => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            ApplicationError::DisallowedAction(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg.to_owned())
            }
            ApplicationError::Render(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("unable to render page: {err}"),
            ),
            ApplicationError::Oidc(msg) => (StatusCode::UNAUTHORIZED, msg.to_owned()),
        }
        .into_response()
    }
}
