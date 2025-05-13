use std::path::PathBuf;

use url::Url;

/// The configuration required to run the application.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct ApplicationConfiguration {
    /// The parent directory to all the `potree` projects being served.
    pub projects_dir: PathBuf,

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
    pub idp_url: Url,
    pub redirect_url: Url,
    pub client_id: String,
    pub client_secret: String,
    pub groups_claim: String,
}
