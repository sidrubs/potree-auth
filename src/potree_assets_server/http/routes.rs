use std::path::PathBuf;

use axum::extract::Path;
use serde::Deserialize;

use super::extractors::PotreeAssets;
use crate::common::domain::StaticAsset;
use crate::common::utils::axum::api_error::ApiError;

#[derive(Deserialize)]
pub(crate) struct Params {
    pub path: PathBuf,
}

/// Serves a built, static `potree` asset,
pub(crate) async fn potree_asset(
    Path(Params { path }): Path<Params>,
    PotreeAssets(potree_assets): PotreeAssets,
) -> Result<StaticAsset, ApiError> {
    Ok(potree_assets.request_asset(&path).await?)
}
