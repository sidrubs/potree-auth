use askama::Template;
use axum::{
    extract::{Path, Request},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::{
    application::{
        extractors::{authorization::Authorization, project::Projects, user::UserExtractor},
        utils::potree::potree_config_path,
        views::potree_render::PotreeRender,
    },
    domain::value_objects::ProjectId,
    error::ApplicationError,
    services::authorization::{Action, Resource},
};

use super::{STATIC_POTREE, login_route};

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
        let login_route = format!("{}?next_path={}", login_route(), request.uri());
        return Ok(Redirect::to(&login_route).into_response());
    }

    // Handle any other auth decisions.
    auth_decision?;

    Ok(Html(
        PotreeRender {
            project_title: project.name,
            potree_config_path: potree_config_path(&project.id)
                .to_string_lossy()
                .to_string(),
            potree_static_assets_path: STATIC_POTREE.to_owned(),
        }
        .render()?,
    )
    .into_response())
}
