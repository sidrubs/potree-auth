use axum::extract::FromRequestParts;
use http::request::Parts;

use super::state::State;
use crate::{
    common::utils::axum::api_error::ApiError,
    potree_assets_server::application::service::PotreeAssetService,
};

pub struct PotreeAssets(pub PotreeAssetService);

/// Defines how `axum` should extract the application [`State`] from the request
/// extensions.
impl<S> FromRequestParts<S> for State
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let state = parts
            .extensions
            .get::<State>()
            .ok_or(ApiError::StateExtraction)?;

        Ok(state.clone())
    }
}

/// Defines how `axum` should extract the [`PotreeAssetStore`] from a request.
impl<S> FromRequestParts<S> for PotreeAssets
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_request_parts(parts, state).await?;
        Ok(Self(state.potree_asset_service))
    }
}
