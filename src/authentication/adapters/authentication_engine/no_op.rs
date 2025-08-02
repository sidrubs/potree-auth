use async_trait::async_trait;

use super::super::super::domain::AuthorizeData;
use super::super::super::domain::CallbackRequestParams;
use super::super::super::domain::OidcSessionPersisted;
use super::super::super::ports::authentication_engine::AuthenticationEngine;
use crate::authentication::ports::authentication_engine::AuthenticationEngineError;
use crate::common::domain::User;

/// A blank authentication service. Would be used if the application does not
/// require authentication.
#[derive(Debug, Clone)]
pub(crate) struct NoOpAuthenticationService;

#[async_trait]
impl AuthenticationEngine for NoOpAuthenticationService {
    #[tracing::instrument]
    async fn authorize(&self) -> Result<AuthorizeData, AuthenticationEngineError> {
        Err(AuthenticationEngineError::Infrastructure {
            message:
                "this is a placeholder for when the application does not require authentication"
                    .to_owned(),
        })
    }

    #[tracing::instrument]
    async fn callback(
        &self,
        _callback_params: CallbackRequestParams,
        _persisted_data: OidcSessionPersisted,
    ) -> Result<User, AuthenticationEngineError> {
        Err(AuthenticationEngineError::Infrastructure {
            message:
                "this is a placeholder for when the application does not require authentication"
                    .to_owned(),
        })
    }
}
