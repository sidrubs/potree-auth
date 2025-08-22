use std::fmt::Debug;

use super::super::domain::action::Action;
use super::super::domain::error::AuthorizationEngineError;
use super::super::domain::resource::Resource;
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
