#[derive(Debug, Clone, thiserror::Error)]
pub enum PotreeAuthError {
    #[error("unable to initialize {adaptor_name} application: {message}")]
    AdaptorIntialization {
        adaptor_name: String,
        message: String,
    },
}
