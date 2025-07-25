use std::path::PathBuf;

use axum::extract::Path;
use serde::Deserialize;

use crate::domain::StaticAsset;
use crate::error::ApplicationError;
use crate::http::extractors::potree_assets::PotreeAssets;

#[derive(Deserialize)]
pub(crate) struct Params {
    path: PathBuf,
}

/// Serves a built, static `potree` asset,
pub(crate) async fn potree_asset(
    Path(Params { path }): Path<Params>,
    PotreeAssets(potree_assets): PotreeAssets,
) -> Result<StaticAsset, ApplicationError> {
    potree_assets.get_asset(&path.to_string_lossy()).await
}
