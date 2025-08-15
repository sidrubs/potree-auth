//! A pretty rudimentary authorization service that implements the
//! [`AuthorizationEngine`] trait.
//!
//! This could be replaced by more robust policy engine in the future.

use crate::common::domain::user::User;
use crate::common::ports::authorization_engine::Action;
use crate::common::ports::authorization_engine::AuthorizationEngine;
use crate::common::ports::authorization_engine::AuthorizationEngineError;
use crate::common::ports::authorization_engine::Resource;

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
            // Read access for Project, ProjectAsset, PotreeRender
            (Resource::Project(project), Action::Read)
            | (Resource::ProjectAsset(project), Action::Read)
            | (Resource::PotreeRender(project), Action::Read) => {
                if user.is_admin()
                    || project
                        .groups
                        .iter()
                        .any(|group| user.groups.contains(group))
                {
                    Ok(())
                } else {
                    Err(AuthorizationEngineError::NotAuthorized {
                        user: Box::new(user.clone()),
                        resource_name: project.name.clone().into(),
                        resource_type: Box::new(resource.into()),
                    })
                }
            }

            // ProjectType list is allowed
            (Resource::ProjectType, Action::List) => Ok(()),

            // Other actions for Project, ProjectAsset, PotreeRender
            (Resource::Project(project), Action::List)
            | (Resource::Project(project), Action::Write)
            | (Resource::Project(project), Action::Update)
            | (Resource::Project(project), Action::Delete)
            | (Resource::ProjectAsset(project), Action::List)
            | (Resource::ProjectAsset(project), Action::Write)
            | (Resource::ProjectAsset(project), Action::Update)
            | (Resource::ProjectAsset(project), Action::Delete)
            | (Resource::PotreeRender(project), Action::List)
            | (Resource::PotreeRender(project), Action::Write)
            | (Resource::PotreeRender(project), Action::Update)
            | (Resource::PotreeRender(project), Action::Delete) => {
                Err(AuthorizationEngineError::NotAuthorized {
                    user: Box::new(user.clone()),
                    resource_name: project.name.clone().into(),
                    resource_type: Box::new(resource.into()),
                })
            }

            // ProjectType with any action except List
            (Resource::ProjectType, Action::Read)
            | (Resource::ProjectType, Action::Write)
            | (Resource::ProjectType, Action::Update)
            | (Resource::ProjectType, Action::Delete) => {
                Err(AuthorizationEngineError::NotAuthorized {
                    user: Box::new(user.clone()),
                    resource_name: "project list".to_owned(),
                    resource_type: Box::new(resource.into()),
                })
            }
        }
    }
}

impl AuthorizationEngine for SimpleAuthorizationEngine {
    fn can(
        &self,
        user: &Option<User>,
        action: &Action,
        resource: &Resource,
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

    mod can {
        use super::*;

        mod project_read {

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

            #[test]
            fn should_return_err_if_the_user_is_not_authenticated() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let user = None;
                let project = Faker.fake::<Project>();

                let resource = Resource::Project(&project);

                // Act
                let res = authorization_service.assert_allowed(&user, &resource, &Action::Read);

                // Assert
                assert!(matches!(
                    res,
                    Err(AuthorizationEngineError::NotAuthenticated)
                ))
            }
        }

        mod project_asset_read {

            use super::*;

            #[test]
            fn should_return_ok_if_the_user_is_an_admin() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let user = User::dummy_admin();
                let project = Faker.fake::<Project>();

                let resource = Resource::ProjectAsset(&project);

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

                let resource = Resource::ProjectAsset(&project);

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

                let resource = Resource::ProjectAsset(&project);

                // Act
                let res =
                    authorization_service.assert_allowed(&Some(user), &resource, &Action::Read);

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

                let user = None;
                let project = Faker.fake::<Project>();

                let resource = Resource::ProjectAsset(&project);

                // Act
                let res = authorization_service.assert_allowed(&user, &resource, &Action::Read);

                // Assert
                assert!(matches!(
                    res,
                    Err(AuthorizationEngineError::NotAuthenticated)
                ))
            }
        }

        mod potree_render_read {

            use super::*;

            #[test]
            fn should_return_ok_if_the_user_is_an_admin() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let user = User::dummy_admin();
                let project = Faker.fake::<Project>();

                let resource = Resource::PotreeRender(&project);

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

                let resource = Resource::PotreeRender(&project);

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

                let resource = Resource::PotreeRender(&project);

                // Act
                let res =
                    authorization_service.assert_allowed(&Some(user), &resource, &Action::Read);

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

                let user = None;
                let project = Faker.fake::<Project>();

                let resource = Resource::PotreeRender(&project);

                // Act
                let res = authorization_service.assert_allowed(&user, &resource, &Action::Read);

                // Assert
                assert!(matches!(
                    res,
                    Err(AuthorizationEngineError::NotAuthenticated)
                ))
            }
        }

        mod project_type_list {
            use super::*;

            #[test]
            fn should_return_ok_if_the_user_is_an_admin() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let user = User::dummy_admin();
                let resource = Resource::ProjectType;

                // Act
                let res =
                    authorization_service.assert_allowed(&Some(user), &resource, &Action::List);

                // Assert
                assert!(res.is_ok())
            }

            #[test]
            fn should_return_ok_for_an_authenticated_user() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let user = Faker.fake::<User>();

                let resource = Resource::ProjectType;

                // Act
                let res =
                    authorization_service.assert_allowed(&Some(user), &resource, &Action::List);

                // Assert
                assert!(res.is_ok())
            }

            #[test]
            fn should_return_err_if_the_user_is_not_authenticated() {
                // Arrange
                let authorization_service = SimpleAuthorizationEngine;

                let user = None;

                let resource = Resource::ProjectType;

                // Act
                let res = authorization_service.assert_allowed(&user, &resource, &Action::List);

                // Assert
                assert!(matches!(
                    res,
                    Err(AuthorizationEngineError::NotAuthenticated)
                ))
            }
        }
    }
}
