use crate::common::utils::http::render_error::RenderError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum PotreeAuthHttpError {
    #[error("unable to initialize {adapter_name} application: {message}")]
    AdapterIntialization {
        adapter_name: String,
        message: String,
    },

    #[error("the server is not configured correctly: {message}")]
    ServerConfiguration { message: String },

    /// These should not be experienced or handled by this router.
    #[error("a runtime error was experienced: {message}")]
    Runtime { message: String },
}

impl From<RenderError> for PotreeAuthHttpError {
    fn from(value: RenderError) -> Self {
        match value {
            RenderError::ServerConfiguration { message } => Self::ServerConfiguration { message },
            RenderError::StateExtraction
            | RenderError::ResourceNotFound { .. }
            | RenderError::NotAuthorized { .. }
            | RenderError::NotAuthenticated
            | RenderError::ServerError { .. }
            | RenderError::AuthenticationFlow { .. }
            | RenderError::Infrastucture { .. } => Self::Runtime {
                message: value.to_string(),
            },
        }
    }
}
