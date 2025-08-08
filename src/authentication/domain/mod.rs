use openidconnect::AuthorizationCode;
use openidconnect::CsrfToken;
use openidconnect::Nonce;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

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
