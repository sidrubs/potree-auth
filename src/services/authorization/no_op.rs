//! An authorization service that allows all requests. It implements the
//! [`AuthorizationService`] trait.
//!
//! This would be used in cases where authorization is not needed.

use crate::{domain::user::User, error::ApplicationError};

use super::{Action, AuthorizationService, Resource};

#[derive(Debug, Clone)]
pub(crate) struct NoOpAuthorizationService;

impl NoOpAuthorizationService {
    #[tracing::instrument]
    pub fn assert_allowed(
        &self,
        _user: &Option<User>,
        _resource: &Resource,
        _action: &Action,
    ) -> Result<(), ApplicationError> {
        Ok(())
    }
}

impl AuthorizationService for NoOpAuthorizationService {
    fn assert_allowed(
        &self,
        user: &Option<User>,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), ApplicationError> {
        Self::assert_allowed(self, user, resource, action)
    }
}
