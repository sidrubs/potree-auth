use crate::domain::ResourceType;

/// Describes all the errors that can be expected by the application.
#[derive(Debug, thiserror::Error)]
#[cfg_attr(test, derive(fake::Dummy))]
pub enum ApplicationError {
    /// Indicates that the user is not authorized to perform a specific action.
    #[error("{user_name} is not authorized to view the {resource_type}: {resource_name}")]
    NotAuthorized {
        user_name: String,
        resource_name: String,
        resource_type: ResourceType,
    },

    /// Indicates that a specific resource was not found.
    #[error("unable to find {resource_type}: {resource_name}")]
    ResourceNotFound {
        resource_name: String,
        resource_type: ResourceType,
    },

    /// A generic server error.
    #[error("{0}")]
    ServerError(String),
}
