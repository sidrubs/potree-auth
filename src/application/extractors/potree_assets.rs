use std::sync::Arc;

use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::{
    application::routes::state::ApplicationState, error::ApplicationError,
    services::potree_assets::PotreeAssetService,
};

pub(crate) struct PotreeAssets(pub(crate) Arc<dyn PotreeAssetService>);

/// Defines how `axum` should extract the [`PotreeAssetService`] from a request.
impl<S> FromRequestParts<S> for PotreeAssets
where
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApplicationState::from_request_parts(parts, state).await?;
        let potree_assets_service = state.potree_asset_service;

        Ok(Self(potree_assets_service))
    }
}
