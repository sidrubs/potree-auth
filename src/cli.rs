//! Contains CLI logic that is called from the main binary.

use std::path::PathBuf;

use clap::Parser;

use crate::config::ApplicationConfiguration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The directory containing the `potree` projects to be served.
    #[arg(short, long, value_name = "PATH")]
    pub projects_dir: PathBuf,
}

impl From<Cli> for ApplicationConfiguration {
    fn from(value: Cli) -> Self {
        let Cli { projects_dir } = value;

        Self { projects_dir }
    }
}
