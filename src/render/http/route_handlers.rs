use askama::Template;
use axum::extract::OriginalUri;
use axum::extract::Path;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

use super::super::application::service::RenderingService;
use super::extractors::LoginRoute;
use super::router::PotreePathParams;
use crate::common::utils::axum::extractors::user::UserExtractor;
use crate::common::utils::axum::render_error::RenderError;
use crate::render::application::error::RenderingServiceError;
use crate::render::http::utils::redirect_to_login;

/// Renders a `potree` project.
pub(crate) async fn potree_render(
    Path(PotreePathParams { project_id }): Path<PotreePathParams>,
    UserExtractor(user): UserExtractor,
    project_rendering_service: RenderingService,
    LoginRoute(login_route): LoginRoute,
    OriginalUri(page_uri): OriginalUri,
) -> Result<Response, RenderError> {
    let res = project_rendering_service
        .render_potree(&user, &project_id)
        .await;

    // Redirect the user agent to the login route if they are not authenticated.
    if let Err(RenderingServiceError::NotAuthenticated) = res {
        return Ok(redirect_to_login(&login_route, page_uri.path()).into_response());
    }

    let potree_template = res?;

    Ok(Html(potree_template.render()?).into_response())
}

/// Displays a dashboard of all the projects a user is allowed to read.
pub(crate) async fn project_dashboard(
    UserExtractor(user): UserExtractor,
    project_rendering_service: RenderingService,
    LoginRoute(login_route): LoginRoute,
    OriginalUri(page_uri): OriginalUri,
) -> Result<Response, RenderError> {
    let res = project_rendering_service.project_dashboard(&user).await;

    // Redirect the user agent to the login route if they are not authenticated.
    if let Err(RenderingServiceError::NotAuthenticated) = res {
        return Ok(redirect_to_login(&login_route, page_uri.path()).into_response());
    }

    let project_dashboard = res?;

    Ok(Html(project_dashboard.render()?).into_response())
}
