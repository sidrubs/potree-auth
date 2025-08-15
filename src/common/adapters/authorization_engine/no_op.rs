//! An authorization service that allows all requests. It implements the
//! [`AuthorizationEngine`] trait.
//!
//! This would be used in cases where authorization is not needed.

use crate::common::domain::user::User;
use crate::common::ports::authorization_engine::Action;
use crate::common::ports::authorization_engine::AuthorizationEngine;
use crate::common::ports::authorization_engine::AuthorizationEngineError;
use crate::common::ports::authorization_engine::Resource;

#[derive(Debug, Clone)]
pub(crate) struct NoOpAuthorizationEngine;

impl NoOpAuthorizationEngine {
    #[tracing::instrument]
    pub fn assert_allowed(
        &self,
        _user: &Option<User>,
        _resource: &Resource,
        _action: &Action,
    ) -> Result<(), AuthorizationEngineError> {
        Ok(())
    }
}

impl AuthorizationEngine for NoOpAuthorizationEngine {
    fn can(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &Resource,
    ) -> Result<(), AuthorizationEngineError> {
        Self::assert_allowed(self, user, resource, action)
    }
}
