use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::application::routes::state::ApplicationState;
use crate::error::ApplicationError;

/// Defines how `axum` should extract the application state from the request
/// extensions.
impl<S> FromRequestParts<S> for ApplicationState
where
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let state = parts
            .extensions
            .get::<ApplicationState>()
            .ok_or(ApplicationError::StateExtraction)?;

        Ok(state.clone())
    }
}
