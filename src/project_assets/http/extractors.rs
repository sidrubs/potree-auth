use axum::extract::FromRequestParts;
use http::request::Parts;

use super::super::application::service::ProjectAssetService;
use super::state::State;
use crate::common::utils::axum::api_error::ApiError;

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

impl<S> FromRequestParts<S> for ProjectAssetService
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_request_parts(parts, state).await?;
        Ok(state.project_asset_service)
    }
}
