use std::fmt::Debug;

use async_trait::async_trait;

use crate::common::domain::user::User;
use crate::error::ApplicationError;
use openidconnect::AuthorizationCode;
use openidconnect::CsrfToken;
use openidconnect::Nonce;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

/// Defines the functionality that needs to be implemented for the application
/// to perform OIDC authentication.
#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait AuthenticationService: Debug + Send + Sync + 'static {
    /// Called as part of the OIDC [`/authorize`] endpoint.
    ///
    /// [`/authorize`]: https://openid.net/specs/openid-connect-core-1_0.html#AuthorizationEndpoint
    async fn authorize(&self) -> Result<AuthorizeData, ApplicationError>;

    /// After authentication, the IdP would redirect the user agent to the
    /// callback route. This would handle the finalizing the OIDC flow to return
    /// the validated user.
    ///
    /// The `persisted_data` would be extracted from the web session; it
    /// should have been stored as part of the `/authorize` endpoint. The
    /// `callback_params` would have come from the query params of the callback
    /// handler.
    async fn callback(
        &self,
        callback_params: CallbackRequestParams,
        persisted_data: OidcSessionPersisted,
    ) -> Result<User, ApplicationError>;
}

/// OIDC data generated from the OIDC Authentication Request (`/authorize`
/// endpoint).
///
/// The documentation on each field indicates its purpose.
#[derive(Debug, Clone)]
pub struct AuthorizeData {
    /// The url to which the user agent should be redirected to perform IdP
    /// authentication.
    pub auth_url: Url,

    /// Information that should be persisted so that it is available whn
    /// handling the callback from the IdP.
    pub persisted_data: OidcSessionPersisted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcSessionPersisted {
    /// The OIDC state. Should be persisted for validation in the callback
    /// handler.
    pub state: CsrfToken,

    /// The OIDC nonce. Should be persisted for validation in the callback
    /// handler.
    pub nonce: Nonce,
}

/// The query params that are supplied in the callback url from the IdP.
///
/// > **Note:** [`Deserialize`] is implemented so that this can be easily
/// > extracted from query params.
#[derive(Debug, Clone, Deserialize)]
pub struct CallbackRequestParams {
    pub code: AuthorizationCode,
    pub state: CsrfToken,
}
