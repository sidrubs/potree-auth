use super::super::ports::authentication_engine::AuthenticationEngineError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthenticationServiceError {
    #[error("unable to set up infrastructure: {message}")]
    Infrastructure { message: String },

    #[error("unable exchange information with the IdP: {message}")]
    IdpExchange { message: String },

    #[error("unable to validate IdP data: {message}")]
    Validation { message: String },
}

impl From<AuthenticationEngineError> for AuthenticationServiceError {
    fn from(value: AuthenticationEngineError) -> Self {
        match value {
            AuthenticationEngineError::Infrastructure { message } => {
                Self::Infrastructure { message }
            }
            AuthenticationEngineError::IdpExchange { message } => Self::IdpExchange { message },
            AuthenticationEngineError::Validation { message } => Self::Validation { message },
        }
    }
}
