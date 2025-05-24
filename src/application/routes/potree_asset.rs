use std::path::PathBuf;

use axum::extract::Path;
use serde::Deserialize;

use crate::{
    application::extractors::potree_assets::PotreeAssets, domain::StaticAsset,
    error::ApplicationError,
};

#[derive(Deserialize)]
pub(crate) struct Params {
    asset_path: PathBuf,
}

/// Serves a built, static `potree` asset,
pub(crate) async fn potree_asset(
    Path(Params { asset_path }): Path<Params>,
    PotreeAssets(potree_assets): PotreeAssets,
) -> Result<StaticAsset, ApplicationError> {
    potree_assets.get_asset(&asset_path.to_string_lossy()).await
}
