use std::path::PathBuf;

use axum::extract::Path;
use http::HeaderMap;
use serde::Deserialize;

use crate::{
    application::extractors::project_assets::ProjectAssets, domain::StaticAsset,
    error::ApplicationError,
};

#[derive(Deserialize)]
pub(crate) struct Params {
    project_id: String,
    path: PathBuf,
}

/// Serves a static project asset.
pub(crate) async fn project_asset(
    Path(Params { project_id, path }): Path<Params>,
    headers: HeaderMap,
    ProjectAssets(project_assets): ProjectAssets,
) -> Result<StaticAsset, ApplicationError> {
    let asset_path = PathBuf::new().join(project_id).join(path);

    project_assets
        .get_asset(&asset_path.to_string_lossy(), Some(headers))
        .await
}
