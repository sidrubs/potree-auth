//! Details about a user of the application.

use std::collections::HashSet;

use super::group::Group;

/// Represents an authenticated user of the application.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct User {
    #[cfg_attr(test, dummy(faker = "fake::faker::name::en::Name()"))]
    pub name: String,
    #[cfg_attr(test, dummy(faker = "fake::faker::internet::en::FreeEmail()"))]
    pub email: String,

    /// The groups that a user is member of, thus has access to their respective
    /// projects.
    pub groups: HashSet<Group>,
}

impl User {
    /// Determines if the user is an admin.
    pub fn is_admin(&self) -> bool {
        self.groups.contains(&Group::new("admin"))
    }
}

#[cfg(test)]
impl User {
    pub fn dummy_admin() -> Self {
        use fake::{Fake, Faker};

        Self {
            groups: [Group::new("admin"), Faker.fake()].into(),
            ..Faker.fake()
        }
    }
}

#[cfg(test)]
mod user_tests {
    use fake::{Fake, Faker};

    use super::*;

    mod is_admin {
        use super::*;

        #[test]
        fn should_return_true_if_user_is_part_of_admin_group() {
            // Arrange
            let user = User::dummy_admin();

            // Act
            let res = user.is_admin();

            // Assert
            assert!(res);
        }

        #[test]
        fn should_return_false_if_user_is_not_part_of_admin_group() {
            // Arrange
            let user = User { ..Faker.fake() };

            // Act
            let res = user.is_admin();

            // Assert
            assert!(!res);
        }
    }
}
