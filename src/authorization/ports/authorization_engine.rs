use std::fmt::Debug;

use super::super::domain::action::Action;
use super::super::domain::error::AuthorizationEngineError;
use crate::authorization::domain::resource::Resource;
use crate::authorization::domain::resource::ResourceInstance;
use crate::common::domain::user::User;

/// Defines the functionality that needs to be implemented for the application
/// to perform authentication.
#[cfg_attr(test, mockall::automock)]
pub trait AuthorizationEngine: Debug + Send + Sync + 'static {
    /// Determines if a `user` should be authorized to perform the `action` on
    /// the specified resource type (type-level).
    ///
    /// # Errors
    ///
    /// - [`AuthorizationEngineError::NotAuthorized`] is returned if the `user`
    ///   is not authorized.
    /// - [`AuthorizationEngineError::NotAuthenticated`] is returned if the
    ///   `user` is `None`, unless the implementation allows unauthenticated
    ///   users.
    fn can_on_type(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &dyn Resource,
    ) -> Result<(), AuthorizationEngineError>;

    /// Determines if a `user` should be authorized to perform the `action` on
    /// the specified resource (instance-level).
    ///
    /// # Errors
    ///
    /// - [`AuthorizationEngineError::NotAuthorized`] is returned if the `user`
    ///   is not authorized.
    /// - [`AuthorizationEngineError::NotAuthenticated`] is returned if the
    ///   `user` is `None`, unless the implementation allows unauthenticated
    ///   users.
    fn can_on_instance(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &dyn ResourceInstance,
    ) -> Result<(), AuthorizationEngineError>;
}
