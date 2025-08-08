use std::sync::Arc;

use super::super::domain::AuthorizeData;
use super::super::domain::CallbackRequestParams;
use super::super::domain::OidcSessionPersisted;
use super::super::ports::authentication_engine::AuthenticationEngine;
use super::error::AuthenticationServiceError;
use crate::common::domain::User;

/// A service for handling user authentication with OIDC.
#[derive(Debug, Clone)]
pub struct AuthenticationService {
    authentication_engine: Arc<dyn AuthenticationEngine>,
}

impl AuthenticationService {
    pub fn new(authentication_engine: Arc<dyn AuthenticationEngine>) -> Self {
        Self {
            authentication_engine,
        }
    }

    //// Called as part of the OIDC [`/authorize`] endpoint.
    ///
    /// [`/authorize`]: https://openid.net/specs/openid-connect-core-1_0.html#AuthorizationEndpoint
    pub async fn authorize(&self) -> Result<AuthorizeData, AuthenticationServiceError> {
        Ok(self.authentication_engine.authorize().await?)
    }

    /// After authentication, the IdP would redirect the user agent to the
    /// callback route. This would handle the finalizing the OIDC flow to return
    /// the validated user.
    ///
    /// The `persisted_data` would be extracted from the web session; it
    /// should have been stored as part of the `/authorize` endpoint. The
    /// `callback_params` would have come from the query params of the callback
    /// handler.
    pub async fn callback(
        &self,
        callback_params: CallbackRequestParams,
        persisted_data: OidcSessionPersisted,
    ) -> Result<User, AuthenticationServiceError> {
        Ok(self
            .authentication_engine
            .callback(callback_params, persisted_data)
            .await?)
    }
}
