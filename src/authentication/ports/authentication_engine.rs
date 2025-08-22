use std::fmt::Debug;

use async_trait::async_trait;

use super::super::domain::AuthorizeData;
use super::super::domain::CallbackRequestParams;
use super::super::domain::OidcSessionPersisted;
use crate::user::domain::User;

/// Defines the functionality that needs to be implemented for the application
/// to perform OIDC authentication.
#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait AuthenticationEngine: Debug + Send + Sync + 'static {
    /// Called as part of the OIDC [`/authorize`] endpoint.
    ///
    /// [`/authorize`]: https://openid.net/specs/openid-connect-core-1_0.html#AuthorizationEndpoint
    async fn authorize(&self) -> Result<AuthorizeData, AuthenticationEngineError>;

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
    ) -> Result<User, AuthenticationEngineError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthenticationEngineError {
    #[error("unable to set up infrastructure: {message}")]
    Infrastructure { message: String },

    #[error("unable exchange information with the IdP: {message}")]
    IdpExchange { message: String },

    #[error("unable to validate IdP data: {message}")]
    Validation { message: String },
}
