use async_trait::async_trait;
use openidconnect::AdditionalClaims;
use openidconnect::Client;
use openidconnect::ClientId;
use openidconnect::ClientSecret;
use openidconnect::CsrfToken;
use openidconnect::EmptyExtraTokenFields;
use openidconnect::EndpointMaybeSet;
use openidconnect::EndpointNotSet;
use openidconnect::EndpointSet;
use openidconnect::IdTokenFields;
use openidconnect::IssuerUrl;
use openidconnect::Nonce;
use openidconnect::RedirectUrl;
use openidconnect::StandardErrorResponse;
use openidconnect::StandardTokenResponse;
use openidconnect::TokenResponse;
use openidconnect::core::CoreAuthDisplay;
use openidconnect::core::CoreAuthPrompt;
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::core::CoreErrorResponseType;
use openidconnect::core::CoreGenderClaim;
use openidconnect::core::CoreJsonWebKey;
use openidconnect::core::CoreJweContentEncryptionAlgorithm;
use openidconnect::core::CoreJwsSigningAlgorithm;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::core::CoreRevocableToken;
use openidconnect::core::CoreRevocationErrorResponse;
use openidconnect::core::CoreTokenIntrospectionResponse;
use openidconnect::core::CoreTokenType;
use serde::Deserialize;
use serde::Serialize;

use super::super::super::ports::authentication_engine::AuthenticationEngine;
use super::super::super::ports::authentication_engine::AuthorizeData;
use super::super::super::ports::authentication_engine::CallbackRequestParams;
use super::super::super::ports::authentication_engine::OidcSessionPersisted;
use super::utils::extract_user_email;
use super::utils::extract_user_groups;
use super::utils::extract_user_id;
use super::utils::extract_user_name;
use crate::authentication::ports::authentication_engine::AuthenticationEngineError;
use crate::common::domain::User;
use crate::error::ApplicationError;

#[derive(Debug, Clone)]
pub(crate) struct OidcAuthenticationEngine {
    /// The oidc client that is performing interaction with the IdP.
    oidc_client: PotreeAuthClient,

    /// The name of the OIDC claim containing and array of
    /// groups that a user is part of.
    groups_claim: String,
}

impl OidcAuthenticationEngine {
    /// Creates a new [`OidcAuthenticationService`] instance with the specified
    /// `projects_directory`.
    ///
    /// # Arguments
    ///
    /// - `idp_url`: The URL to the IdP service.
    /// - `redirect_url`: The URL to which the IdP should redirect the
    ///   user-agent after successful authentication.
    /// - `client_id`: The `id` of the application on the IdP.
    /// - `client_secret`: The Authorization Code Flow client secret shared
    ///   between the IdP and the application.
    /// - `groups_claim`: The name of the OIDC claim containing and array of
    ///   groups that a user is part of.
    pub async fn new(
        idp_url: IssuerUrl,
        redirect_url: RedirectUrl,
        client_id: ClientId,
        client_secret: ClientSecret,
        groups_claim: String,
    ) -> Result<Self, AuthenticationEngineError> {
        // Sets up an http client to interact with the IdP
        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| AuthenticationEngineError::Infrastructure {
                message: format!("unable to build OIDC http client: {e}"),
            })?;

        // Request the oidc config from the IdP.
        let provider_metadata = CoreProviderMetadata::discover_async(idp_url, &http_client)
            .await
            .map_err(|e| AuthenticationEngineError::IdpExchange {
                message: format!("unable to perform OIDC discovery: {e}"),
            })?;

        let oidc_client = PotreeAuthClient::from_provider_metadata(
            provider_metadata,
            client_id,
            Some(client_secret),
        )
        .set_redirect_uri(redirect_url);

        Ok(Self {
            oidc_client,
            groups_claim,
        })
    }

    /// Initiates an OIDC authentication flow with the IdP.
    ///
    /// Returns the `auth_url` needed to redirect the user agent to the IdP's
    /// OIDC `/authorize` route. And the `persisted_data` that is needed to
    /// complete the authentication flow in the callback handler.
    #[tracing::instrument]
    async fn login(&self) -> Result<AuthorizeData, AuthenticationEngineError> {
        let (auth_url, state, nonce) = self
            .oidc_client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .url();

        Ok(AuthorizeData {
            auth_url,
            persisted_data: OidcSessionPersisted { state, nonce },
        })
    }

    /// Finalizes the OIDC authentication flow with the IdP.
    ///
    /// Returns the authenticated [`User`].
    #[tracing::instrument]
    async fn callback(
        &self,
        callback_params: CallbackRequestParams,
        persisted_data: OidcSessionPersisted,
    ) -> Result<User, AuthenticationEngineError> {
        // Sets up an http client to interact with the IdP
        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| AuthenticationEngineError::Infrastructure {
                message: format!("unable to build OIDC http client: {e}"),
            })?;

        // Verify that state matches that from the session
        if callback_params.state.secret() != persisted_data.state.secret() {
            return Err(AuthenticationEngineError::Validation {
                message: "the session and request `state`s do not match".to_owned(),
            });
        }

        // Request the tokens from the IdP
        let token_response = self
            .oidc_client
            .exchange_code(callback_params.code)
            .map_err(|e| AuthenticationEngineError::Infrastructure {
                message: format!("unable to initialize OIDC code exchange client: {e}"),
            })?
            .request_async(&http_client)
            .await
            .map_err(|e| AuthenticationEngineError::IdpExchange {
                message: format!("unable to perform token request: {e}"),
            })?;

        // Extract the claims from the id token.
        let id_token_claims = token_response
            .id_token()
            .ok_or(AuthenticationEngineError::Validation {
                message: "IdP did not return id_token".to_owned(),
            })?
            .claims(&self.oidc_client.id_token_verifier(), &persisted_data.nonce)
            .map_err(|err| AuthenticationEngineError::Validation {
                message: format!("unable to extract claims from id_token: {err}"),
            })?;

        Ok(User {
            id: extract_user_id(id_token_claims),
            name: extract_user_name(id_token_claims)?,
            email: extract_user_email(id_token_claims)?,
            groups: extract_user_groups(id_token_claims, &self.groups_claim),
        })
    }
}

#[async_trait]
impl AuthenticationEngine for OidcAuthenticationEngine {
    async fn authorize(&self) -> Result<AuthorizeData, AuthenticationEngineError> {
        Self::login(self).await
    }

    async fn callback(
        &self,
        callback_params: CallbackRequestParams,
        persisted_data: OidcSessionPersisted,
    ) -> Result<User, AuthenticationEngineError> {
        Self::callback(self, callback_params, persisted_data).await
    }
}

pub type PotreeAuthTokenFields = IdTokenFields<
    PotreeAuthClaims,
    EmptyExtraTokenFields,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
>;

pub type PotreeAuthTokenResponse = StandardTokenResponse<PotreeAuthTokenFields, CoreTokenType>;

type PotreeAuthClient<
    HasAuthUrl = EndpointSet,
    HasDeviceAuthUrl = EndpointNotSet,
    HasIntrospectionUrl = EndpointNotSet,
    HasRevocationUrl = EndpointNotSet,
    HasUserInfoUrl = EndpointMaybeSet,
    HasTokenUrl = EndpointMaybeSet,
> = Client<
    PotreeAuthClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    PotreeAuthTokenResponse,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
    HasAuthUrl,
    HasDeviceAuthUrl,
    HasIntrospectionUrl,
    HasRevocationUrl,
    HasTokenUrl,
    HasUserInfoUrl,
>;

/// The claims from the IdP that are required to extract a [`User`].
///
/// The custom claims are required because we don't know the id token claims
/// that represent their associated groups at compile time (i.e. they are in the
/// config).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PotreeAuthClaims(pub serde_json::Value);

impl AdditionalClaims for PotreeAuthClaims {}
