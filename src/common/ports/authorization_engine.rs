use std::fmt::Debug;

use crate::common::domain::ResourceType;
use crate::common::domain::project::Project;
use crate::common::domain::user::User;

/// Defines the functionality that needs to be implemented for the application
/// to perform authentication.
#[cfg_attr(test, mockall::automock)]
pub trait AuthorizationEngine: Debug + Send + Sync + 'static {
    /// Determines if a `user` should be authorized to perform the `action` on
    /// the specified resource.
    ///
    /// # Errors
    ///
    /// - [`AuthorizationEngineError::NotAuthorized`] is returned if the `user`
    ///   is not authorized.
    /// - [`AuthorizationEngineError::NotAuthenticated`] is returned if the
    ///   `user` is `None`, unless the implementation allows unauthenticated
    ///   users.
    #[allow(
        clippy::needless_lifetimes,
        reason = "it seems mockall need the explicit lifetime declaration"
    )]
    fn assert_allowed<'a>(
        &self,
        user: &Option<User>,
        resource: &Resource<'a>,
        action: &Action,
    ) -> Result<(), AuthorizationEngineError>;
}

/// Defines a resource that can be accessed.
#[derive(Debug, Clone)]
pub enum Resource<'a> {
    Project(&'a Project),
}

/// Defines actions that can be performed on a [`Resource`].
#[derive(Debug, Clone)]
pub enum Action {
    Read,
    List,
    Write,
    Update,
    Delete,
}

impl From<&Resource<'_>> for ResourceType {
    fn from(value: &Resource) -> Self {
        match value {
            Resource::Project(_) => Self::Project,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthorizationEngineError {
    #[error("{} is not authorized to view the {:?}: {}", user.name, resource_type, resource_name)]
    NotAuthorized {
        user: User,
        resource_name: String,
        resource_type: ResourceType,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,
}
