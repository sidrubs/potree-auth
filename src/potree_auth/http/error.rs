use crate::common::utils::http::initialization_error::InitializationError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum PotreeAuthHttpError {
    #[error("unable to initialize `{adapter_name}` application: {message}")]
    AdapterIntialization {
        adapter_name: String,
        message: String,
    },

    #[error("unable to initialize `{middleware_name}` middleware: {message}")]
    MiddlewareIntialization {
        middleware_name: String,
        message: String,
    },

    #[error("the server is not configured correctly: {message}")]
    ServerConfiguration { message: String },
}

impl From<InitializationError> for PotreeAuthHttpError {
    fn from(value: InitializationError) -> Self {
        match value {
            InitializationError::Middleware {
                middleware_name,
                message,
            } => Self::MiddlewareIntialization {
                middleware_name,
                message,
            },
        }
    }
}
