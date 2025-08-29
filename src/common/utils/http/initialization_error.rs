/// Errors that can be experienced when initializing an `axum` router (i.e. not
/// when handling requests).
#[derive(Debug, Clone, thiserror::Error)]
pub enum InitializationError {
    #[error("unable to initialize `{middleware_name}` middleware: {message}")]
    Middleware {
        middleware_name: String,
        message: String,
    },
}
