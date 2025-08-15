//! Contains CLI logic that is called from the main binary.

use std::path::PathBuf;

use clap::Parser;
use url::Url;

use super::config::IdpConfiguration as PotreeAuthIdpConfiguration;
use super::config::PotreeAuthConfiguration;

#[derive(Debug, Clone, Parser)]
#[command(version, about = None, long_about = None)]
pub struct Cli {
    /// The parent directory containing the projects to be served.
    #[arg(short, long, env = "DATA_DIR")]
    pub data_dir: PathBuf,

    /// If populated will use an OIDC IdP for authentication, else won't use
    /// authentication.
    #[clap(flatten)]
    pub idp: Option<IdpConfiguration>,

    /// Configures how the server should behave.
    #[clap(flatten)]
    pub server: ServerConfiguration,
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
#[group(requires_all = ["idp_url", "idp_client_id", "idp_client_secret", "idp_groups_claim", "idp_application_external_url"])]
pub struct IdpConfiguration {
    /// The URL to the OIDC IdP.
    #[arg(long, required = false, env = "IDP_URL")]
    pub idp_url: Url,

    /// The id of the application on the IdP.
    #[arg(long, required = false, env = "IDP_CLIENT_ID")]
    pub idp_client_id: String,

    /// The Authorization Code Flow client secret shared between the IdP and the
    /// application.
    #[arg(long, required = false, env = "IDP_CLIENT_SECRET")]
    pub idp_client_secret: String,

    /// The claim in the OIDC Id Token that will contain an array of the groups
    /// that the authenticated user is member of.
    #[arg(long, required = false, env = "IDP_GROUPS_CLAIM")]
    pub idp_groups_claim: String,

    /// The URL on which the application is publicly accessible (the OIDC
    /// callback URL is calculated from this).
    #[arg(long, required = false, env = "IDP_APPLICATION_EXTERNAL_URL")]
    pub idp_application_external_url: Url,
}

/// Configures server specific controls.
#[derive(Debug, Clone, clap::Args)]
pub struct ServerConfiguration {
    /// The interface on which the application is listening.
    ///
    /// Commonly `localhost`/`127.0.0.1` to serve on the local machine or
    /// `0.0.0.0` to serve on all interfaces.
    #[arg(long, default_value = "127.0.0.1", env = "SERVER_HOST")]
    pub host: String,

    /// The port on which the server should be listening.
    #[arg(long, default_value_t = 3000, env = "SERVER_PORT")]
    pub port: u16,
}

impl From<Cli> for PotreeAuthConfiguration {
    fn from(value: Cli) -> Self {
        let Cli { data_dir, idp, .. } = value;

        Self {
            data_dir,
            idp: idp.map(Into::into),
        }
    }
}

impl From<IdpConfiguration> for PotreeAuthIdpConfiguration {
    fn from(value: IdpConfiguration) -> Self {
        let IdpConfiguration {
            idp_url,
            idp_client_id,
            idp_client_secret,
            idp_groups_claim,
            idp_application_external_url,
        } = value;

        Self {
            idp_url,
            client_id: idp_client_id,
            client_secret: idp_client_secret,
            groups_claim: idp_groups_claim,
            external_url: idp_application_external_url,
        }
    }
}
