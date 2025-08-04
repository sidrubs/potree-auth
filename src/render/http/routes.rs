use askama::Template;
use axum::extract::Path;
use axum::response::Html;

use super::super::application::service::RenderingService;
use super::router::PotreePathParams;
use crate::common::utils::axum::extractors::user::UserExtractor;
use crate::common::utils::axum::render_error::RenderError;

/// Serves a static `project` asset.
pub(crate) async fn potree_render(
    Path(PotreePathParams { project_id }): Path<PotreePathParams>,
    UserExtractor(user): UserExtractor,
    project_rendering_service: RenderingService,
) -> Result<Html<String>, RenderError> {
    let potree_template = project_rendering_service
        .render_potree(&user, &project_id)
        .await?;

    Ok(Html(potree_template.render()?))
}
