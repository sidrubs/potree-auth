use axum::extract::Query;
use axum::response::Redirect;
use tower_sessions::Session;
use web_route::WebRoute;

use super::super::application::service::AuthenticationService;
use super::super::domain::AuthorizeData;
use super::super::domain::CallbackRequestParams;
use super::super::domain::OidcSessionPersisted;
use crate::common::utils::http::render_error::RenderError;
use crate::user::http::extractors::USER_SESSION_KEY;

/// The key to which login data will be stored in a session.
pub const LOGIN_SESSION_KEY: &str = "login_session";

/// The data to be persisted between the [`login`] and [`callback`] routes.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct LoginSessionData {
    oidc_persisted_data: OidcSessionPersisted,
    next_path: WebRoute,
}

#[derive(serde::Deserialize)]
pub(crate) struct LoginParams {
    /// The path to which the user should be redirected after
    /// logging in.
    next_path: WebRoute,
}

/// Initiates an OIDC login flow with the configured IdP.
pub(crate) async fn login(
    Query(LoginParams { next_path }): Query<LoginParams>,
    session: Session,
    authentication_service: AuthenticationService,
) -> Result<Redirect, RenderError> {
    // Clear anything in the current session as we are setting up a new login.
    session.clear().await;
    session
        .cycle_id()
        .await
        .map_err(|_e| RenderError::AuthenticationFlow {
            message: "unable to reset session".to_owned(),
        })?;

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
        .map_err(|_e| RenderError::AuthenticationFlow {
            message: "unable to persist OIDC data".to_owned(),
        })?;

    Ok(Redirect::to(auth_url.as_str()))
}

/// Initiates an OIDC login flow with the configured IdP.
pub(crate) async fn callback(
    Query(callback_params): Query<CallbackRequestParams>,
    session: Session,
    authentication_service: AuthenticationService,
) -> Result<Redirect, RenderError> {
    // Retrieve data that was persisted in the `login` route.
    let LoginSessionData {
        oidc_persisted_data,
        next_path,
    } = session
        .get::<LoginSessionData>(LOGIN_SESSION_KEY)
        .await
        .map_err(|_e| RenderError::AuthenticationFlow {
            message: "unable to retrieve OIDC data".to_owned(),
        })?
        .ok_or(RenderError::AuthenticationFlow {
            message: "no matching session data".to_owned(),
        })?;

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
        .map_err(|_e| RenderError::AuthenticationFlow {
            message: "unable to persist user data in the session".to_owned(),
        })?;

    Ok(Redirect::to(&next_path))
}
