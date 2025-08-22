use openidconnect::IdTokenClaims;
use openidconnect::core::CoreGenderClaim;

use super::oidc::PotreeAuthClaims;
use crate::authentication::ports::authentication_engine::AuthenticationEngineError;
use crate::common::domain::Group;
use crate::user::domain::EmailAddress;
use crate::user::domain::UserId;
use crate::user::domain::UserName;

pub(crate) fn extract_user_groups(
    id_token_claims: &IdTokenClaims<PotreeAuthClaims, CoreGenderClaim>,
    groups_claim: &str,
) -> Vec<Group> {
    if let Some(serde_json::Value::Array(arr)) =
        &id_token_claims.additional_claims().0.get(groups_claim)
    {
        arr.iter()
            .filter_map(|v| v.as_str().map(Group::new))
            .collect::<Vec<_>>()
    } else {
        tracing::debug!("no groups claim ({}) found, setting to []", groups_claim);

        Vec::new()
    }
}

pub(crate) fn extract_user_id(
    id_token_claims: &IdTokenClaims<PotreeAuthClaims, CoreGenderClaim>,
) -> UserId {
    UserId::new(id_token_claims.subject().to_string())
}

pub(crate) fn extract_user_name(
    id_token_claims: &IdTokenClaims<PotreeAuthClaims, CoreGenderClaim>,
) -> Result<UserName, AuthenticationEngineError> {
    Ok(UserName::new(
        id_token_claims
            .name()
            .ok_or(AuthenticationEngineError::Validation {
                message: "no `name` associated with user".to_owned(),
            })?
            .get(None)
            .ok_or(AuthenticationEngineError::Validation {
                message: "no `name` associated with user".to_owned(),
            })?
            .to_string(),
    ))
}

pub(crate) fn extract_user_email(
    id_token_claims: &IdTokenClaims<PotreeAuthClaims, CoreGenderClaim>,
) -> Result<EmailAddress, AuthenticationEngineError> {
    Ok(EmailAddress::new(
        id_token_claims
            .email()
            .ok_or(AuthenticationEngineError::Validation {
                message: "no `email` associated with user".to_owned(),
            })?
            .to_string(),
    ))
}
