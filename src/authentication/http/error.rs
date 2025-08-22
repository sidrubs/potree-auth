use super::super::application::error::AuthenticationServiceError;
use crate::common::utils::http::render_error::RenderError;

impl From<AuthenticationServiceError> for RenderError {
    fn from(value: AuthenticationServiceError) -> Self {
        match value {
            AuthenticationServiceError::Infrastructure { message }
            | AuthenticationServiceError::IdpExchange { message }
            | AuthenticationServiceError::Validation { message } => {
                Self::AuthenticationFlow { message }
            }
        }
    }
}
