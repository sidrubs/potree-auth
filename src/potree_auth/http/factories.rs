use std::sync::Arc;

use super::super::config::IdpConfiguration;
use super::error::PotreeAuthHttpError;
use super::router::AUTH;
use crate::authentication::adapters::authentication_engine::no_op::NoOpAuthenticationEngine;
use crate::authentication::adapters::authentication_engine::oidc::OidcAuthenticationEngine;
use crate::authentication::http::CALLBACK;
use crate::authentication::ports::authentication_engine::AuthenticationEngine;
use crate::authorization::adapters::basic_authorization::SimpleAuthorizationEngine;
use crate::authorization::adapters::no_op::NoOpAuthorizationEngine;
use crate::authorization::ports::authorization_engine::AuthorizationEngine;

/// Initialize an authentication engine to handle OIDC authentication.
///
/// If no `idp_config` is provided, a no-op authentication engine is returned.
pub async fn init_authentication_engine(
    idp_config: Option<IdpConfiguration>,
) -> Result<Arc<dyn AuthenticationEngine>, PotreeAuthHttpError> {
    Ok(if let Some(idp_config) = idp_config {
        let redirect_url = idp_config
            .external_url
            .join(&AUTH.join(CALLBACK.as_ref()))
            .map_err(|e| PotreeAuthHttpError::AdapterIntialization {
                adapter_name: "OidcAuthenticationEngine".to_owned(),
                message: format!("unable to build redirect url: {e}"),
            })?;

        let authentication_engine = OidcAuthenticationEngine::new(
            idp_config.idp_url,
            redirect_url,
            idp_config.client_id,
            idp_config.client_secret,
            idp_config.groups_claim,
        )
        .await
        .map_err(|e| PotreeAuthHttpError::AdapterIntialization {
            adapter_name: "OidcAuthenticationEngine".to_owned(),
            message: e.to_string(),
        })?;

        Arc::new(authentication_engine)
    } else {
        Arc::new(NoOpAuthenticationEngine)
    })
}

/// Initializes the authorization engine.
///
/// If `authentication_configured` (e.g. OIDC) then a valid authorization engine
/// will be used. Else a no-op engine will be used that allow unauthenticated
/// users to have access.
pub fn init_authorization_engine(authentication_configured: bool) -> Arc<dyn AuthorizationEngine> {
    if authentication_configured {
        Arc::new(SimpleAuthorizationEngine)
    } else {
        Arc::new(NoOpAuthorizationEngine)
    }
}
