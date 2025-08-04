use axum::extract::Query;
use axum::response::Redirect;
use serde::Deserialize;
use serde::Serialize;
use tower_sessions::Session;
use web_route::WebRoute;

use crate::error::ApplicationError;
use crate::http::extractors::authentication::Authentication;
use crate::http::utils::auth::USER_SESSION_KEY;
use crate::services::authentication_service::AuthorizeData;
use crate::services::authentication_service::CallbackRequestParams;
use crate::services::authentication_service::OidcSessionPersisted;

/// The key to which login data will be stored in a session.
pub const LOGIN_SESSION_KEY: &str = "login_session";

/// The data to be persisted between the [`login`] and [`callback`] routes.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LoginSessionData {
    oidc_persisted_data: OidcSessionPersisted,
    next_path: WebRoute,
}

#[derive(Deserialize)]
pub(crate) struct LoginParams {
    /// The path to which the user should be redirected after
    /// logging in.
    next_path: WebRoute,
}

/// Initiates an OIDC login flow with the configured IdP.
pub(crate) async fn login(
    Query(LoginParams { next_path }): Query<LoginParams>,
    session: Session,
    Authentication(authentication_service): Authentication,
) -> Result<Redirect, ApplicationError> {
    // Clear anything in the current session as we are setting up a new login.
    session.clear().await;
    session
        .cycle_id()
        .await
        .map_err(|_err| ApplicationError::ServerError("unable to reset session".to_owned()))?;

    // Generate OIDC login data
    let AuthorizeData {
        auth_url,
        persisted_data: oidc_persisted_data,
    } = authentication_service.authorize().await?;

    // Persist the required data so that it is available for the `callback` route to
    // complete the login.
    session
        .insert(
            LOGIN_SESSION_KEY,
            LoginSessionData {
                oidc_persisted_data,
                next_path,
            },
        )
        .await
        .map_err(|_err| ApplicationError::ServerError("unable to persist OIDC data".to_owned()))?;

    Ok(Redirect::to(auth_url.as_str()))
}

/// Initiates an OIDC login flow with the configured IdP.
pub(crate) async fn callback(
    Query(callback_params): Query<CallbackRequestParams>,
    session: Session,
    Authentication(authentication_service): Authentication,
) -> Result<Redirect, ApplicationError> {
    // Retrieve data that was persisted in the `login` route.
    let LoginSessionData {
        oidc_persisted_data,
        next_path,
    } = session
        .get::<LoginSessionData>(LOGIN_SESSION_KEY)
        .await
        .map_err(|_err| ApplicationError::ServerError("unable to retrieve OIDC data".to_owned()))?
        .ok_or(ApplicationError::Oidc(
            "no matching session data".to_owned(),
        ))?;

    // Get authenticated user.
    let user = authentication_service
        .callback(callback_params, oidc_persisted_data)
        .await?;

    // Clear the existing OIDC data from the session
    session.clear().await;

    // Insert the user into the session to log them in.
    session
        .insert(USER_SESSION_KEY, user)
        .await
        .map_err(|_err| ApplicationError::ServerError("unable to persist user data".to_owned()))?;

    Ok(Redirect::to(&next_path))
}
