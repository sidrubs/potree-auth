use askama::Template;
use axum::extract::OriginalUri;
use axum::extract::Path;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

use super::super::application::service::RenderingService;
use super::extractors::LoginRoute;
use super::router::PotreePathParams;
use crate::common::utils::http::render_error::RenderError;
use crate::render::application::error::RenderingServiceError;
use crate::render::http::router::POTREE;
use crate::render::http::utils::redirect_to_404;
use crate::render::http::utils::redirect_to_login;
use crate::user::http::extractors::UserExtractor;

/// Renders a `potree` project.
#[tracing::instrument(name = "`rendering route handlers`: rendering potree project", err)]
pub async fn potree_render(
    Path(PotreePathParams { project_id }): Path<PotreePathParams>,
    UserExtractor(user): UserExtractor,
    rendering_service: RenderingService,
    LoginRoute(login_route): LoginRoute,
    OriginalUri(page_uri): OriginalUri,
) -> Result<Response, RenderError> {
    let res = rendering_service.render_potree(&user, &project_id).await;

    // Redirect the user agent to the login route if they are not authenticated.
    if let Err(RenderingServiceError::NotAuthenticated) = res {
        tracing::warn!("user not authenticated, redirecting to login");
        return Ok(redirect_to_login(&login_route, page_uri.path()).into_response());
    }

    // Redirect user to 404 page if the project can't be found.
    if let Err(RenderingServiceError::ProjectNotFound { .. }) = res {
        tracing::error!(project_id = ?project_id, "project not found");
        return Ok(redirect_to_404().into_response());
    }

    // Redirect user to 404 page if the user is not authorized to view the project.
    if let Err(RenderingServiceError::NotAuthorized { .. }) = res {
        tracing::error!(user = ?user, project_id = ?project_id, "user not authorized to render project");
        return Ok(redirect_to_404().into_response());
    }

    let potree_template = res?;

    Ok(Html(potree_template.render()?).into_response())
}

/// Displays a dashboard of all the projects a user is allowed to read.
#[tracing::instrument(name = "`rendering route handlers`: rendering project dashboard", err)]
pub async fn project_dashboard(
    UserExtractor(user): UserExtractor,
    rendering_service: RenderingService,
    LoginRoute(login_route): LoginRoute,
    OriginalUri(page_uri): OriginalUri,
) -> Result<Response, RenderError> {
    let res = rendering_service.project_dashboard(&user, &POTREE).await;

    // Redirect the user agent to the login route if they are not authenticated.
    if let Err(RenderingServiceError::NotAuthenticated) = res {
        tracing::warn!("user not authenticated, redirecting to login");
        return Ok(redirect_to_login(&login_route, page_uri.path()).into_response());
    }

    // Redirect user to 404 page if the user is not authorized to view the project
    // dashboard.
    if let Err(RenderingServiceError::NotAuthorized { .. }) = res {
        tracing::error!(user = ?user, "user not authorized to view project dashboard");
        return Ok(redirect_to_404().into_response());
    }

    let project_dashboard = res?;

    Ok(Html(project_dashboard.render()?).into_response())
}

/// Display a 404 page.
#[tracing::instrument(name = "`rendering route handlers`: rendering 404", err)]
pub async fn not_found(rendering_service: RenderingService) -> Result<Response, RenderError> {
    let not_found = rendering_service.not_found().await?;

    Ok(Html(not_found.render()?).into_response())
}
