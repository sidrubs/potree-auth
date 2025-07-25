use std::sync::Arc;

use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::http::ApplicationError;
use crate::http::routes::state::ApplicationState;
use crate::services::potree_asset_store::PotreeAssetStore;

pub(crate) struct PotreeAssets(pub(crate) Arc<dyn PotreeAssetStore>);

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
