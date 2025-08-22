//! A pretty rudimentary authorization service that implements the
//! [`AuthorizationEngine`] trait.
//!
//! This could be replaced by more robust policy engine in the future.

use super::super::domain::action::Action;
use super::super::domain::error::AuthorizationEngineError;
use super::super::domain::resource::Resource;
use super::super::domain::resource::ResourceInstance;
use super::super::ports::authorization_engine::AuthorizationEngine;
use crate::common::domain::user::User;

/// Handles authorization business logic for the application.
#[derive(Debug, Clone)]
pub(crate) struct SimpleAuthorizationEngine;

impl SimpleAuthorizationEngine {
    #[tracing::instrument(
        name = "`simple_authorization_engine`: evaluating on resource type",
        err
    )]
    pub fn can_on_type(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &dyn Resource,
    ) -> Result<(), AuthorizationEngineError> {
        // If there is no user then the user is not authenticated.
        let Some(user) = user else {
            return Err(AuthorizationEngineError::NotAuthenticated);
        };

        // An admin should always be allowed to action.
        if user.is_admin() {
            return Ok(());
        };

        match (action, resource.resource_type().as_str()) {
            (&Action::List, "project") => Ok(()),
            (&Action::Read, "project_dashboard") => Ok(()),
            _ => Err(AuthorizationEngineError::NotAuthorized {
                user: Box::new(user.clone()),
                action: action.clone(),
                resource_identifier: None,
                resource_type: resource.resource_type(),
            }),
        }
    }

    #[tracing::instrument(
        name = "`simple_authorization_engine`: evaluating on resource instance",
        err
    )]
    pub fn can_on_instance(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &dyn ResourceInstance,
    ) -> Result<(), AuthorizationEngineError> {
        // If there is no user then the user is not authenticated.
        let Some(user) = user else {
            return Err(AuthorizationEngineError::NotAuthenticated);
        };

        // Allows a user to _read_ any resource of which they share a group.
        match action {
            &Action::Read => {
                if user.is_admin()
                    || resource
                        .groups()
                        .iter()
                        .any(|group| user.groups.contains(group))
                {
                    Ok(())
                } else {
                    Err(AuthorizationEngineError::NotAuthorized {
                        user: Box::new(user.clone()),
                        action: action.clone(),
                        resource_identifier: Some(resource.resource_identifier()),
                        resource_type: resource.resource_type(),
                    })
                }
            }
            _ => Err(AuthorizationEngineError::NotAuthorized {
                user: Box::new(user.clone()),
                action: action.clone(),
                resource_identifier: Some(resource.resource_identifier()),
                resource_type: resource.resource_type(),
            }),
        }
    }
}

impl AuthorizationEngine for SimpleAuthorizationEngine {
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

#[cfg(test)]
mod authorization_service_tests {
    use fake::Fake;
    use fake::Faker;

    use super::super::super::domain::resource::mocked_resource::MockedResource;
    use super::*;
    use crate::common::domain::group::Group;

    mod can_on_type {

        use crate::authorization::domain::resource::ResourceType;

        use super::*;

        #[test]
        fn should_return_ok_if_the_user_is_an_admin() {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let user = User::dummy_admin();
            let resource = Faker.fake::<MockedResource>();

            // Act
            let res = authorization_service.can_on_type(&Some(user), &Action::Read, &resource);

            // Assert
            assert!(res.is_ok())
        }

        #[test]
        fn should_return_ok_if_listing_projects() {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let user = Faker.fake::<User>();
            let resource = MockedResource {
                resource_type: ResourceType::new("project".to_owned()),
                ..Faker.fake()
            };
            let action = Action::List;

            // Act
            let res = authorization_service.can_on_type(&Some(user), &action, &resource);

            // Assert
            assert!(res.is_ok())
        }

        #[test_case::test_matrix(
            [&Action::Create, &Action::Read, &Action::Update, &Action::Delete]
            )]
        fn should_return_err_if_not_listing_projects(action: &Action) {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let user = Faker.fake::<User>();
            let resource = MockedResource {
                resource_type: ResourceType::new("project".to_owned()),
                ..Faker.fake()
            };

            // Act
            let res = authorization_service.can_on_type(&Some(user), action, &resource);

            // Assert
            assert!(matches!(
                res,
                Err(AuthorizationEngineError::NotAuthorized { .. })
            ))
        }

        #[test]
        fn should_return_ok_if_reading_a_project_dashboard() {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let user = Faker.fake::<User>();
            let resource = MockedResource {
                resource_type: ResourceType::new("project_dashboard".to_owned()),
                ..Faker.fake()
            };
            let action = Action::Read;

            // Act
            let res = authorization_service.can_on_type(&Some(user), &action, &resource);

            // Assert
            assert!(res.is_ok())
        }

        #[test_case::test_matrix(
            [&Action::Create, &Action::List, &Action::Update, &Action::Delete]
            )]
        fn should_return_err_if_not_reading_a_project_dashboard(action: &Action) {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let user = Faker.fake::<User>();
            let resource = MockedResource {
                resource_type: ResourceType::new("project_dashboard".to_owned()),
                ..Faker.fake()
            };

            // Act
            let res = authorization_service.can_on_type(&Some(user), action, &resource);

            // Assert
            assert!(matches!(
                res,
                Err(AuthorizationEngineError::NotAuthorized { .. })
            ))
        }

        #[test]
        fn should_return_err_for_other_combinations() {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let user = Faker.fake::<User>();
            let resource = Faker.fake::<MockedResource>();
            let action = Faker.fake::<Action>();

            // Act
            let res = authorization_service.can_on_type(&Some(user), &action, &resource);

            // Assert
            assert!(matches!(
                res,
                Err(AuthorizationEngineError::NotAuthorized { .. })
            ))
        }
    }

    mod can_on_instance {

        use super::*;

        #[test]
        fn should_return_ok_if_the_user_is_an_admin() {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let user = User::dummy_admin();
            let resource = Faker.fake::<MockedResource>();

            // Act
            let res = authorization_service.can_on_instance(&Some(user), &Action::Read, &resource);

            // Assert
            assert!(res.is_ok())
        }

        #[test]
        fn should_return_ok_if_the_user_shares_a_group_with_the_resource() {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let shared_group = Faker.fake::<Group>();
            let user = User {
                groups: [Faker.fake(), shared_group.clone()].into(),
                ..Faker.fake()
            };
            let resource = MockedResource {
                groups: vec![shared_group],
                ..Faker.fake()
            };

            // Act
            let res = authorization_service.can_on_instance(&Some(user), &Action::Read, &resource);

            // Assert
            assert!(res.is_ok())
        }

        #[test]
        fn should_return_err_if_the_user_does_not_share_a_group_with_the_resource() {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let user = Faker.fake::<User>();
            let resource = MockedResource {
                groups: vec![],
                ..Faker.fake()
            };

            // Act
            let res = authorization_service.can_on_instance(&Some(user), &Action::Read, &resource);

            // Assert
            assert!(matches!(
                res,
                Err(AuthorizationEngineError::NotAuthorized { .. })
            ))
        }

        #[test]
        fn should_return_err_if_the_user_is_not_authenticated() {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            // Act
            let res = authorization_service.can_on_instance(
                &None,
                &Action::Read,
                &Faker.fake::<MockedResource>(),
            );

            // Assert
            assert!(matches!(
                res,
                Err(AuthorizationEngineError::NotAuthenticated)
            ))
        }

        #[test_case::test_case(&Action::List; "list")]
        #[test_case::test_case(&Action::Create; "create")]
        #[test_case::test_case(&Action::Update; "update")]
        #[test_case::test_case(&Action::Delete; "delete")]
        fn should_return_err_if_anything_other_than_read(action: &Action) {
            // Arrange
            let authorization_service = SimpleAuthorizationEngine;

            let shared_group = Faker.fake::<Group>();
            let user = User {
                groups: [Faker.fake(), shared_group.clone()].into(),
                ..Faker.fake()
            };
            let resource = MockedResource {
                groups: vec![shared_group],
                ..Faker.fake()
            };

            // Act
            let res = authorization_service.can_on_instance(&Some(user), action, &resource);

            // Assert
            assert!(matches!(
                res,
                Err(AuthorizationEngineError::NotAuthorized { .. })
            ))
        }
    }
}
