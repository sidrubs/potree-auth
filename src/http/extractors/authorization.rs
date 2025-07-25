use std::sync::Arc;

use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::http::ApplicationError;
use crate::http::routes::state::ApplicationState;
use crate::services::authorization_engine::AuthorizationEngine;

pub(crate) struct Authorization(pub(crate) Arc<dyn AuthorizationEngine>);

/// Defines how `axum` should extract the [`AuthorizationService`] from a
/// request.
impl<S> FromRequestParts<S> for Authorization
where
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApplicationState::from_request_parts(parts, state).await?;
        let authorization_service = state.authorization_service;

        Ok(Self(authorization_service))
    }
}
