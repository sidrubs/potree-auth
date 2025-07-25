use crate::common::domain::ResourceType;

/// Describes all the errors that can be expected by the application.
#[derive(Debug, thiserror::Error)]
// #[cfg_attr(test, derive(fake::Dummy))]
pub enum ApplicationError {
    /// Indicates that the user is not authorized to perform a specific action.
    #[error("{user_name} is not authorized to view the {resource_type}: {resource_name}")]
    NotAuthorized {
        user_name: String,
        resource_name: String,
        resource_type: ResourceType,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,

    /// Indicates that a specific resource was not found.
    #[error("unable to find {resource_type}: {resource_name}")]
    ResourceNotFound {
        resource_name: String,
        resource_type: ResourceType,
    },

    /// A generic server error.
    #[error("{0}")]
    ServerError(String),

    /// A disallowed action was attempted.
    #[error("{0}")]
    DisallowedAction(String),

    /// Occurs if unable to extract application state from the request parts.
    #[error("unable to extract application state; ensure it was added as a router extension")]
    StateExtraction,

    /// An error experienced rendering a page.
    #[error(transparent)]
    Render(#[from] askama::Error),

    /// Error in the OIDC flow.
    #[error("{0}")]
    Oidc(String),

    /// An error experienced during application initialization.
    #[error("{0}")]
    Initialization(String),
}
