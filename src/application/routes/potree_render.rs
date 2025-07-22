use askama::Template;
use axum::extract::Path;
use axum::extract::Request;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::response::Response;
use serde::Deserialize;

use crate::application::extractors::authorization::Authorization;
use crate::application::extractors::project::Projects;
use crate::application::extractors::user::UserExtractor;
use crate::application::routes::AUTH_LOGIN;
use crate::application::routes::POTREE_ASSETS_ROOT;
use crate::application::utils::potree::potree_config_path;
use crate::application::views::potree_render::PotreeRender;
use crate::domain::value_objects::ProjectId;
use crate::error::ApplicationError;
use crate::services::authorization_engine::Action;
use crate::services::authorization_engine::Resource;

#[derive(Deserialize)]
pub(crate) struct Params {
    project_id: ProjectId,
}

/// Renders a `potree` page,
pub(crate) async fn potree_render(
    Authorization(authorization_service): Authorization,
    Path(Params { project_id }): Path<Params>,
    Projects(project_service): Projects,
    UserExtractor(user): UserExtractor,
    request: Request,
) -> Result<Response, ApplicationError> {
    let project = project_service.read(&project_id).await?;

    let auth_decision =
        authorization_service.assert_allowed(&user, &Resource::Project(&project), &Action::Read);

    // If not authenticated, redirect the user to the login page.
    if matches!(&auth_decision, &Err(ApplicationError::NotAuthenticated)) {
        let login_route = format!("{}?next_path={}", *AUTH_LOGIN, request.uri());
        return Ok(Redirect::to(&login_route).into_response());
    }

    // Handle any other auth decisions.
    auth_decision?;

    Ok(Html(
        PotreeRender {
            project_title: project.name,
            potree_config_path: potree_config_path(&project.id),
            potree_static_assets_path: POTREE_ASSETS_ROOT.to_string(),
        }
        .render()?,
    )
    .into_response())
}
