use crate::{
    domain::{project::Project, user::User},
    error::ApplicationError,
};

pub mod basic_authorization;

/// Defines the functionality that needs to be implemented for the application
/// to perform authentication.
#[cfg_attr(test, mockall::automock)]
pub trait AuthorizationService {
    /// Determines if a `user` should be authorized to perform the `action` on
    /// the specified resource.
    ///
    /// # Errors
    ///
    /// An [`ApplicationError::NotAuthorized`] is returned if the `user` is not
    /// authorized.
    #[expect(
        clippy::needless_lifetimes,
        reason = "it seems mockall need the explicit lifetime declaration"
    )]
    fn assert_allowed<'a>(
        &self,
        user: &User,
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
