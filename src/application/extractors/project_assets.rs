use std::sync::Arc;

use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::{
    application::routes::state::ApplicationState, error::ApplicationError,
    services::project_assets::ProjectAssetService,
};

pub(crate) struct ProjectAssets(pub(crate) Arc<dyn ProjectAssetService>);

/// Defines how `axum` should extract the [`ProjectAssetService`] from a
/// request.
impl<S> FromRequestParts<S> for ProjectAssets
where
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApplicationState::from_request_parts(parts, state).await?;
        let project_assets_service = state.project_asset_service;

        Ok(Self(project_assets_service))
    }
}
