use std::sync::Arc;

use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::http::ApplicationError;
use crate::http::routes::state::ApplicationState;
use crate::services::project_store::ProjectService;

pub(crate) struct Projects(pub(crate) Arc<dyn ProjectService>);

/// Defines how `axum` should extract the [`ProjectService`] from a
/// request.
impl<S> FromRequestParts<S> for Projects
where
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApplicationState::from_request_parts(parts, state).await?;
        let projects_service = state.project_service;

        Ok(Self(projects_service))
    }
}
