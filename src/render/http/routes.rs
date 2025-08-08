use askama::Template;
use axum::extract::OriginalUri;
use axum::extract::Path;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::response::Response;

use super::super::application::service::RenderingService;
use super::extractors::LoginRoute;
use super::router::PotreePathParams;
use crate::common::utils::axum::extractors::user::UserExtractor;
use crate::common::utils::axum::render_error::RenderError;
use crate::render::application::error::RenderingServiceError;

/// Serves a static `project` asset.
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
        return Ok(
            Redirect::to(&format!("{}?next_path={}", login_route, page_uri.path())).into_response(),
        );
    }

    let potree_template = res?;

    Ok(Html(potree_template.render()?).into_response())
}
