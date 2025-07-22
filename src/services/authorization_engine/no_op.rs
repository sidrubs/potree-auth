//! An authorization service that allows all requests. It implements the
//! [`AuthorizationEngine`] trait.
//!
//! This would be used in cases where authorization is not needed.

use crate::{domain::user::User, error::ApplicationError};

use super::{Action, AuthorizationEngine, Resource};

#[derive(Debug, Clone)]
pub(crate) struct NoOpAuthorizationEngine;

impl NoOpAuthorizationEngine {
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

impl AuthorizationEngine for NoOpAuthorizationEngine {
    fn assert_allowed(
        &self,
        user: &Option<User>,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), ApplicationError> {
        Self::assert_allowed(self, user, resource, action)
    }
}
