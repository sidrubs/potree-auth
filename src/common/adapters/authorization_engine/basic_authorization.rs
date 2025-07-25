//! A pretty rudimentary authorization service that implements the
//! [`AuthorizationEngine`] trait.
//!
//! This could be replaced by more robust policy engine in the future.

use crate::common::domain::user::User;
use crate::common::ports::authorization_engine::{
    Action, AuthorizationEngine, AuthorizationEngineError, Resource,
};

/// Handles authorization business logic for the application.
#[derive(Debug, Clone)]
pub(crate) struct SimpleAuthorizationEngine;

impl SimpleAuthorizationEngine {
    #[tracing::instrument]
    pub fn assert_allowed(
        &self,
        user: &Option<User>,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), AuthorizationEngineError> {
        // If there is no user then the user is not authenticated.
        let Some(user) = user else {
            return Err(AuthorizationEngineError::NotAuthenticated);
        };

        // Determine if the user is authorized to access the resource.
        match (resource, action) {
            (Resource::Project(project), Action::Read) => {
                if user.is_admin()
                    || project
                        .groups
                        .iter()
                        .any(|group| user.groups.contains(group))
                {
                    Ok(())
                } else {
                    Err(AuthorizationEngineError::NotAuthorized {
                        user: user.clone(),
                        resource_name: project.name.clone().into(),
                        resource_type: resource.into(),
                    })
                }
            }
            (Resource::Project(project), Action::List)
            | (Resource::Project(project), Action::Write)
            | (Resource::Project(project), Action::Update)
            | (Resource::Project(project), Action::Delete) => {
                Err(AuthorizationEngineError::NotAuthorized {
                    user: user.clone(),
                    resource_name: project.name.clone().into(),
                    resource_type: resource.into(),
                })
            }
        }
    }
}

impl AuthorizationEngine for SimpleAuthorizationEngine {
    fn assert_allowed(
        &self,
        user: &Option<User>,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), AuthorizationEngineError> {
        Self::assert_allowed(self, user, resource, action)
    }
}

#[cfg(test)]
mod authorization_service_tests {
    use fake::Fake;
    use fake::Faker;

    use super::*;
    use crate::common::domain::group::Group;
    use crate::common::domain::project::Project;

    mod assert_allowed {
        use super::*;

        mod project_resource_read {

            use super::*;

            #[test]
            fn should_return_ok_if_the_user_is_an_admin() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let user = User::dummy_admin();
                let project = Faker.fake::<Project>();

                let resource = Resource::Project(&project);

                // Act
                let res =
                    authorization_service.assert_allowed(&Some(user), &resource, &Action::Read);

                // Assert
                assert!(res.is_ok())
            }

            #[test]
            fn should_return_ok_if_the_user_shares_a_group_with_the_project() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let shared_group = Faker.fake::<Group>();
                let user = User {
                    groups: [Faker.fake(), shared_group.clone()].into(),
                    ..Faker.fake()
                };
                let project = Project {
                    groups: [Faker.fake(), shared_group.clone()].into(),
                    ..Faker.fake()
                };

                let resource = Resource::Project(&project);

                // Act
                let res =
                    authorization_service.assert_allowed(&Some(user), &resource, &Action::Read);

                // Assert
                assert!(res.is_ok())
            }

            #[test]
            fn should_return_err_if_the_user_does_not_share_a_group_with_the_project() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let user = Faker.fake::<User>();
                let project = Faker.fake::<Project>();

                let resource = Resource::Project(&project);

                // Act
                let res =
                    authorization_service.assert_allowed(&Some(user), &resource, &Action::Read);

                // Assert
                assert!(matches!(
                    res,
                    Err(AuthorizationEngineError::NotAuthorized { .. })
                ))
            }
        }
    }
}
