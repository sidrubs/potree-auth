use std::fmt::Debug;

use crate::domain::project::Project;
use crate::domain::user::User;
use crate::error::ApplicationError;

pub(crate) mod basic_authorization;
pub(crate) mod no_op;

/// Defines the functionality that needs to be implemented for the application
/// to perform authentication.
#[cfg_attr(test, mockall::automock)]
pub trait AuthorizationEngine: Debug + Send + Sync + 'static {
    /// Determines if a `user` should be authorized to perform the `action` on
    /// the specified resource.
    ///
    /// # Errors
    ///
    /// - [`ApplicationError::NotAuthorized`] is returned if the `user` is not
    ///   authorized.
    /// - [`ApplicationError::NotAuthenticated`] is returned if the `user` is
    ///   `None`.
    #[allow(
        clippy::needless_lifetimes,
        reason = "it seems mockall need the explicit lifetime declaration"
    )]
    fn assert_allowed<'a>(
        &self,
        user: &Option<User>,
        resource: &Resource<'a>,
        action: &Action,
    ) -> Result<(), ApplicationError>;
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
}
