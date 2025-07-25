use axum::extract::FromRequestParts;
use http::request::Parts;
use tower_sessions::Session;

use crate::domain::User;
use crate::error::ApplicationError;
use crate::http::utils::auth::USER_SESSION_KEY;

pub(crate) struct UserExtractor(pub(crate) Option<User>);

/// Defines how `axum` should extract a [`User`] from a request.
///
/// The user is stored in the web session.
impl<S> FromRequestParts<S> for UserExtractor
where
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await.map_err(|_err| ApplicationError::ServerError("could not find tower sessions in request. ensure that tower sessions is in the middleware stack".to_owned()))?;
        let user = session.get::<User>(USER_SESSION_KEY).await.unwrap();

        Ok(Self(user))
    }
}
