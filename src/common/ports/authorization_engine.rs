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
    fn can<'a>(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &Resource<'a>,
    ) -> Result<(), AuthorizationEngineError>;
}

/// Defines a resource that can be accessed.
#[derive(Debug, Clone)]
pub enum Resource<'a> {
    /// A specific project (instance-level).
    Project(&'a Project),

    /// Projects in general (type-level). Usually for `list` actions.
    ProjectType,

    /// An asset associated with a specific project (instance-level).
    ProjectAsset(&'a Project),

    /// Potree rendering for a specific project (instance-level).
    PotreeRender(&'a Project),
}

/// Defines actions that can be performed on a [`Resource`].
#[expect(dead_code, reason = "other actions to be used in the future")]
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
            Resource::Project(_) | Resource::ProjectType => Self::Project,
            Resource::ProjectAsset(_) => Self::ProjectAsset,
            Resource::PotreeRender(_) => Self::PotreeRender,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthorizationEngineError {
    #[error("{} is not authorized to view the {:?}: {}", user.name, resource_type, resource_name)]
    NotAuthorized {
        user: Box<User>,
        resource_name: String,
        resource_type: Box<ResourceType>,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,
}
