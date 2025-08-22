use axum::extract::Path;
use http::HeaderMap;

use super::super::application::service::ProjectAssetService;
use super::router::AssetPathParams;
use crate::common::domain::StaticAsset;
use crate::common::utils::http::api_error::ApiError;
use crate::user::http::extractors::UserExtractor;

/// Serves a static `project` asset.
pub(crate) async fn project_asset(
    Path(AssetPathParams { project_id, path }): Path<AssetPathParams>,
    UserExtractor(user): UserExtractor,
    project_assets: ProjectAssetService,
    headers: HeaderMap,
) -> Result<StaticAsset, ApiError> {
    Ok(project_assets
        .read_asset(&user, &project_id, &path, Some(headers))
        .await?)
}
