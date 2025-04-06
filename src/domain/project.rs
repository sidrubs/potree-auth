use std::collections::HashSet;

use crate::error::ApplicationError;

use super::{group::Group, user::User};

/// Represents the metadata associated with a 3D model project.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct Project {
    pub name: String,

    /// The groups that are allowed to access the project.
    pub groups: HashSet<Group>,
}

impl Project {
    /// Determines if a `user` should be authorized to view a [`Project`].
    ///
    /// # Errors
    ///
    /// An [`ApplicationError::NotAuthorized`] is returned if the `user` is not
    /// authorized.
    pub fn assert_can_view(&self, user: &User) -> Result<(), ApplicationError> {
        if user.is_admin() || self.groups.iter().any(|group| user.groups.contains(group)) {
            Ok(())
        } else {
            Err(ApplicationError::NotAuthorized {
                user_name: user.name.clone(),
                project_name: self.name.clone(),
            })
        }
    }
}

#[cfg(test)]
mod project_tests {
    use super::*;

    mod assert_can_view {
        use fake::{Fake, Faker};

        use super::*;

        #[test]
        fn should_return_ok_if_the_user_is_an_admin() {
            // Arrange
            let user = User::dummy_admin();
            let project = Faker.fake::<Project>();

            // Act
            let res = project.assert_can_view(&user);

            // Assert
            assert!(res.is_ok())
        }

        #[test]
        fn should_return_ok_if_the_user_shares_a_group_with_the_project() {
            // Arrange
            let shared_group = Faker.fake::<Group>();
            let user = User {
                groups: [Faker.fake(), shared_group.clone()].into(),
                ..Faker.fake()
            };
            let project = Project {
                groups: [Faker.fake(), shared_group.clone()].into(),
                ..Faker.fake()
            };

            // Act
            let res = project.assert_can_view(&user);

            // Assert
            assert!(res.is_ok())
        }

        #[test]
        fn should_return_err_if_the_user_does_not_share_a_group_with_the_project() {
            // Arrange
            let user = Faker.fake::<User>();
            let project = Faker.fake::<Project>();

            // Act
            let res = project.assert_can_view(&user);

            // Assert
            assert!(matches!(res, Err(ApplicationError::NotAuthorized { .. })))
        }
    }
}
