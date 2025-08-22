use axum::extract::Path;

use super::super::application::service::PotreeAssetService;
use super::router::AssetPathParams;
use crate::common::domain::StaticAsset;
use crate::common::utils::http::api_error::ApiError;

/// Serves a built, static `potree` asset,
pub(crate) async fn potree_asset(
    Path(AssetPathParams { path }): Path<AssetPathParams>,
    potree_assets: PotreeAssetService,
) -> Result<StaticAsset, ApiError> {
    Ok(potree_assets.request_asset(&path).await?)
}
