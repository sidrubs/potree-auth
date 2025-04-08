/// Describes all the errors that can be expected by the application.
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("{user_name} is not authorized to view the {resource_type}: {resource_name}")]
    NotAuthorized {
        user_name: String,
        resource_name: String,
        resource_type: String,
    },
}
