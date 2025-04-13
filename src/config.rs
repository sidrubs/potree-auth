use std::path::PathBuf;

/// The configuration required to run the application.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct ApplicationConfiguration {
    /// The parent directory to all the `potree` projects being served.
    pub projects_dir: PathBuf,
}
