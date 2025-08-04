use axum::extract::FromRequestParts;
use http::request::Parts;
use tower_sessions::Session;

use crate::common::domain::User;
use crate::common::utils::axum::api_error::ApiError;

/// The key to which the logged in user data will be stored in a session.
pub const USER_SESSION_KEY: &str = "user_session";

pub struct UserExtractor(pub Option<User>);

/// Defines how `axum` should extract a [`User`] from a request.
///
/// The user is stored in the web session.
impl<S> FromRequestParts<S> for UserExtractor
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await.map_err(|_err| ApiError::ServerConfiguration{message:"could not find tower sessions in request - ensure that tower sessions is in the middleware stack".to_owned()})?;
        let user = session.get::<User>(USER_SESSION_KEY).await.unwrap();

        Ok(Self(user))
    }
}
