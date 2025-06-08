//! Contains CLI logic that is called from the main binary.

use std::path::PathBuf;

use clap::Parser;
use url::Url;

use crate::config::{ApplicationConfiguration, IdpConfiguration as ApplicationIdpConfiguration};

#[derive(Debug, Clone, Parser)]
#[command(version, about = None, long_about = None)]
pub struct Cli {
    /// The directory containing the `potree` projects to be served.
    #[arg(short, long, env = "PROJECTS_DIR")]
    pub projects_dir: PathBuf,

    /// If populated will use an OIDC IdP for authentication, else won't use
    /// authentication.
    #[clap(flatten)]
    pub idp: Option<IdpConfiguration>,
}

/// The configuration required to use an OIDC IdP for authentication.
///
/// If this is not provided the application won't require any authentication.
///
/// Note the slightly non-standard `clap` formatting, see [here][1] for more
/// information.
///
/// [1]: https://github.com/clap-rs/clap/issues/5092#issuecomment-2616986075
#[derive(Debug, Clone, clap::Args)]
#[group(requires_all = ["idp_url", "client_id", "client_secret", "groups_claim", "external_url"])]
pub struct IdpConfiguration {
    /// The URL to the OIDC IdP.
    #[arg(long, required = false, env = "IDP_URL")]
    pub idp_url: Url,

    /// The id of the application on the IdP.
    #[arg(long, required = false, env = "CLIENT_ID")]
    pub client_id: String,

    /// The Authorization Code Flow client secret shared between the IdP and the
    /// application.
    #[arg(long, required = false, env = "CLIENT_SECRET")]
    pub client_secret: String,

    /// The claim in the OIDC Id Token that will contain an array of the groups that
    /// the authenticated user is member of.
    #[arg(long, required = false, env = "GROUPS_CLAIM")]
    pub groups_claim: String,

    /// The URL on which the application is publicly accessible (the OIDC
    /// callback URL is calculated from this).
    #[arg(long, required = false, env = "EXTERNAL_URL")]
    pub external_url: Url,
}

impl From<Cli> for ApplicationConfiguration {
    fn from(value: Cli) -> Self {
        let Cli { projects_dir, idp } = value;

        Self {
            projects_dir,
            idp: idp.map(Into::into),
        }
    }
}

impl From<IdpConfiguration> for ApplicationIdpConfiguration {
    fn from(value: IdpConfiguration) -> Self {
        let IdpConfiguration {
            idp_url,
            client_id,
            client_secret,
            groups_claim,
            external_url,
        } = value;

        Self {
            idp_url,
            client_id,
            client_secret,
            groups_claim,
            external_url,
        }
    }
}
