use askama::Template;
use axum::{extract::Path, response::Html};
use serde::Deserialize;

use crate::{
    application::{
        extractors::project::Projects, utils::potree::potree_config_path,
        views::potree_render::PotreeRender,
    },
    domain::value_objects::ProjectId,
    error::ApplicationError,
};

use super::STATIC_POTREE;

#[derive(Deserialize)]
pub(crate) struct Params {
    project_id: ProjectId,
}

/// Renders a `potree` page,
pub(crate) async fn potree_render(
    Path(Params { project_id }): Path<Params>,
    Projects(project_service): Projects,
) -> Result<Html<String>, ApplicationError> {
    let project = project_service.read(&project_id).await?;

    dbg!(potree_config_path(&project.id));

    Ok(Html(
        PotreeRender {
            project_title: project.name,
            potree_config_path: potree_config_path(&project.id)
                .to_string_lossy()
                .to_string(),
            potree_static_assets_path: STATIC_POTREE.to_owned(),
        }
        .render()
        .unwrap(),
    ))
}
