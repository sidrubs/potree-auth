use std::collections::HashSet;

use openidconnect::IdTokenClaims;
use openidconnect::core::CoreGenderClaim;

use super::oidc::PotreeAuthClaims;
use crate::domain::Group;
use crate::domain::value_objects::EmailAddress;
use crate::domain::value_objects::UserId;
use crate::domain::value_objects::UserName;
use crate::error::ApplicationError;

pub(crate) fn extract_user_groups(
    id_token_claims: &IdTokenClaims<PotreeAuthClaims, CoreGenderClaim>,
    groups_claim: &str,
) -> HashSet<Group> {
    if let Some(serde_json::Value::Array(arr)) =
        &id_token_claims.additional_claims().0.get(groups_claim)
    {
        arr.iter()
            .filter_map(|v| v.as_str().map(Group::new))
            .collect::<HashSet<_>>()
    } else {
        tracing::debug!("no groups claim ({}) found, setting to []", groups_claim);

        HashSet::new()
    }
}

pub(crate) fn extract_user_id(
    id_token_claims: &IdTokenClaims<PotreeAuthClaims, CoreGenderClaim>,
) -> UserId {
    UserId::new(id_token_claims.subject().to_string())
}

pub(crate) fn extract_user_name(
    id_token_claims: &IdTokenClaims<PotreeAuthClaims, CoreGenderClaim>,
) -> Result<UserName, ApplicationError> {
    Ok(UserName::new(
        id_token_claims
            .name()
            .ok_or(ApplicationError::Oidc(
                "no `name` associated with user".to_owned(),
            ))?
            .get(None)
            .ok_or(ApplicationError::Oidc(
                "no `name` associated with user".to_owned(),
            ))?
            .to_string(),
    ))
}

pub(crate) fn extract_user_email(
    id_token_claims: &IdTokenClaims<PotreeAuthClaims, CoreGenderClaim>,
) -> Result<EmailAddress, ApplicationError> {
    Ok(EmailAddress::new(
        id_token_claims
            .email()
            .ok_or(ApplicationError::Oidc(
                "no `email` associated with user".to_owned(),
            ))?
            .to_string(),
    ))
}
