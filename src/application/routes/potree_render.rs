use askama::Template;
use axum::{
    extract::{Path, Request},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::{
    application::{
        extractors::{authorization::Authorization, project::Projects, user::UserExtractor},
        routes::{AUTH_LOGIN, POTREE_ASSETS_ROOT},
        utils::potree::potree_config_path,
        views::potree_render::PotreeRender,
    },
    domain::value_objects::ProjectId,
    error::ApplicationError,
    services::authorization::{Action, Resource},
};

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
