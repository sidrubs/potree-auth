use axum::extract::FromRequestParts;
use http::request::Parts;
use web_route::WebRoute;

use super::super::application::service::RenderingService;
use super::state::State;
use crate::common::utils::axum::render_error::RenderError;

impl<S> FromRequestParts<S> for State
where
    S: Send + Sync,
{
    type Rejection = RenderError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let state = parts
            .extensions
            .get::<State>()
            .ok_or(RenderError::StateExtraction)?;

        Ok(state.clone())
    }
}

impl<S> FromRequestParts<S> for RenderingService
where
    S: Send + Sync,
{
    type Rejection = RenderError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_request_parts(parts, state).await?;
        Ok(state.rendering_service)
    }
}

pub struct LoginRoute(pub WebRoute);

impl<S> FromRequestParts<S> for LoginRoute
where
    S: Send + Sync,
{
    type Rejection = RenderError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_request_parts(parts, state).await?;
        Ok(Self(state.login_route))
    }
}
