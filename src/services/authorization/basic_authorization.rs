//! A pretty rudimentary authorization service that implements the
//! [`AuthorizationService`] trait.
//!
//! This could be replaced by more robust policy engine in the future.

use crate::{domain::user::User, error::ApplicationError};

use super::{Action, AuthorizationService, Resource};

/// Handles authorization business logic for the application.
#[derive(Debug, Clone)]
pub(crate) struct SimpleAuthorizationService;

impl SimpleAuthorizationService {
    pub fn assert_allowed(
        &self,
        user: &User,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), ApplicationError> {
        match (resource, action) {
            // Reading a `Project`.
            (Resource::Project(project), Action::Read) => {
                if user.is_admin()
                    || project
                        .groups
                        .iter()
                        .any(|group| user.groups.contains(group))
                {
                    Ok(())
                } else {
                    Err(ApplicationError::NotAuthorized {
                        user_name: user.name.clone().into(),
                        resource_name: project.name.clone().into(),
                        resource_type: resource.into(),
                    })
                }
            }
        }
    }
}

impl AuthorizationService for SimpleAuthorizationService {
    fn assert_allowed(
        &self,
        user: &User,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), ApplicationError> {
        Self::assert_allowed(self, user, resource, action)
    }
}

#[cfg(test)]
mod authorization_service_tests {
    use crate::domain::{group::Group, project::Project};
    use fake::{Fake, Faker};

    use super::*;

    mod assert_allowed {
        use super::*;

        mod project_resource_read {

            use super::*;

            #[test]
            fn should_return_ok_if_the_user_is_an_admin() {
                // Arrange
                let authorization_service = SimpleAuthorizationService;

                let user = User::dummy_admin();
                let project = Faker.fake::<Project>();

                let resource = Resource::Project(&project);

                // Act
                let res = authorization_service.assert_allowed(&user, &resource, &Action::Read);

                // Assert
                assert!(res.is_ok())
            }

            #[test]
            fn should_return_ok_if_the_user_shares_a_group_with_the_project() {
                // Arrange
                let authorization_service = SimpleAuthorizationService;

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
                let res = authorization_service.assert_allowed(&user, &resource, &Action::Read);

                // Assert
                assert!(res.is_ok())
            }

            #[test]
            fn should_return_err_if_the_user_does_not_share_a_group_with_the_project() {
                // Arrange
                let authorization_service = SimpleAuthorizationService;

                let user = Faker.fake::<User>();
                let project = Faker.fake::<Project>();

                let resource = Resource::Project(&project);

                // Act
                let res = authorization_service.assert_allowed(&user, &resource, &Action::Read);

                // Assert
                assert!(matches!(res, Err(ApplicationError::NotAuthorized { .. })))
            }
        }
    }
}
