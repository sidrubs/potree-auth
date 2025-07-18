use std::path::PathBuf;

use url::Url;

/// The configuration required to run the application.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct ApplicationConfiguration {
    /// The parent directory to all the projects being served.
    pub data_dir: PathBuf,

    /// Populated to use an IdP for authentication.
    pub idp: Option<IdpConfiguration>,
}

#[cfg(test)]
impl ApplicationConfiguration {
    pub fn dummy_with_no_idp() -> Self {
        use fake::{Fake, Faker};

        Self {
            idp: None,
            ..Faker.fake()
        }
    }
}

/// The configuration required to use an OIDC IdP for authentication.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct IdpConfiguration {
    /// The URL to the IdP service.
    pub idp_url: Url,

    /// The `id` of the `potree-auth` application on the IdP.
    pub client_id: String,

    /// The Authorization Code Flow client secret shared between the IdP and the
    /// application.
    pub client_secret: String,

    /// The name of the OIDC claim containing and array of groups that a user is
    /// part of.
    pub groups_claim: String,

    /// The URL on which the application is publicly accessible (the OIDC
    /// callback URL is calculated from this).
    pub external_url: Url,
}
