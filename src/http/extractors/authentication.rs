use std::sync::Arc;

use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::http::ApplicationError;
use crate::http::routes::state::ApplicationState;
use crate::services::authentication_service::AuthenticationService;

pub(crate) struct Authentication(pub(crate) Arc<dyn AuthenticationService>);

/// Defines how `axum` should extract the [`AuthenticationService`] from a
/// request.
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApplicationState::from_request_parts(parts, state).await?;
        let authentication_service = state.authentication_service;

        Ok(Self(authentication_service))
    }
}
