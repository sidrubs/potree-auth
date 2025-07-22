use std::path::PathBuf;

use axum::extract::Path;
use http::HeaderMap;
use serde::Deserialize;

use crate::application::extractors::authorization::Authorization;
use crate::application::extractors::project::Projects;
use crate::application::extractors::project_assets::ProjectAssets;
use crate::application::extractors::user::UserExtractor;
use crate::domain::StaticAsset;
use crate::domain::value_objects::ProjectId;
use crate::error::ApplicationError;
use crate::services::authorization_engine::Action;
use crate::services::authorization_engine::Resource;

#[derive(Deserialize)]
pub(crate) struct Params {
    project_id: ProjectId,
    path: PathBuf,
}

/// Serves a static project asset.
pub(crate) async fn project_asset(
    Authorization(authorization_service): Authorization,
    Path(Params { project_id, path }): Path<Params>,
    headers: HeaderMap,
    ProjectAssets(project_assets): ProjectAssets,
    Projects(project_service): Projects,
    UserExtractor(user): UserExtractor,
) -> Result<StaticAsset, ApplicationError> {
    // Ensure that the `user` is allowed to read the project.
    let project = project_service.read(&project_id).await?;
    authorization_service.assert_allowed(&user, &Resource::Project(&project), &Action::Read)?;

    // Built a path to the asset. It would be within the project dir.
    let asset_path = PathBuf::new().join(project_id.as_str()).join(path);

    project_assets
        .get_asset(&asset_path.to_string_lossy(), Some(headers))
        .await
}
