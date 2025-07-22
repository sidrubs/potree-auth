use async_trait::async_trait;

use super::AuthenticationService;
use super::AuthorizeData;
use super::CallbackRequestParams;
use super::OidcSessionPersisted;
use crate::domain::User;
use crate::error::ApplicationError;

/// A blank authentication service. Would be used if the application does not
/// require authentication.
#[derive(Debug, Clone)]
pub(crate) struct NoOpAuthenticationService;

#[async_trait]
impl AuthenticationService for NoOpAuthenticationService {
    #[tracing::instrument]
    async fn authorize(&self) -> Result<AuthorizeData, ApplicationError> {
        Err(ApplicationError::ServerError(
            "this is a placeholder for when the application does not require authentication"
                .to_owned(),
        ))
    }

    #[tracing::instrument]
    async fn callback(
        &self,
        _callback_params: CallbackRequestParams,
        _persisted_data: OidcSessionPersisted,
    ) -> Result<User, ApplicationError> {
        Err(ApplicationError::ServerError(
            "this is a placeholder for when the application does not require authentication"
                .to_owned(),
        ))
    }
}
