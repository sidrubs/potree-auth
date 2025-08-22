//! An authorization service that allows all requests. It implements the
//! [`AuthorizationEngine`] trait.
//!
//! This would be used in cases where authorization is not needed.

use super::super::domain::action::Action;
use super::super::domain::error::AuthorizationEngineError;
use super::super::domain::resource::Resource;
use super::super::domain::resource::ResourceInstance;
use super::super::ports::authorization_engine::AuthorizationEngine;
use crate::user::domain::User;

#[derive(Debug, Clone)]
pub(crate) struct NoOpAuthorizationEngine;

impl NoOpAuthorizationEngine {
    #[tracing::instrument(
        name = "`no_op_authorization_engine`: evaluating on resource type",
        err
    )]
    pub fn can_on_type(
        &self,
        _user: &Option<User>,
        _action: &Action,
        _resource: &dyn Resource,
    ) -> Result<(), AuthorizationEngineError> {
        Ok(())
    }

    #[tracing::instrument(
        name = "`no_op_authorization_engine`: evaluating on resource instance",
        err
    )]
    fn can_on_instance(
        &self,
        _user: &Option<User>,
        _action: &Action,
        _resource: &dyn ResourceInstance,
    ) -> Result<(), AuthorizationEngineError> {
        Ok(())
    }
}

impl AuthorizationEngine for NoOpAuthorizationEngine {
    fn can_on_type(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &dyn Resource,
    ) -> Result<(), AuthorizationEngineError> {
        Self::can_on_type(self, user, action, resource)
    }

    fn can_on_instance(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &dyn ResourceInstance,
    ) -> Result<(), AuthorizationEngineError> {
        Self::can_on_instance(self, user, action, resource)
    }
}
