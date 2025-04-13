use std::path::PathBuf;

use axum::extract::Path;

use crate::{
    application::extractors::potree_assets::PotreeAssets, domain::StaticAsset,
    error::ApplicationError,
};

/// Serves a built, static `potree` asset,
pub(crate) async fn potree_asset(
    Path(asset_path): Path<PathBuf>,
    PotreeAssets(potree_assets): PotreeAssets,
) -> Result<StaticAsset, ApplicationError> {
    potree_assets.get_asset(&asset_path.to_string_lossy()).await
}
