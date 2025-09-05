//! Details about a user of the application.

use serde::Deserialize;
use serde::Serialize;

use crate::common::domain::Group;
use crate::common::domain::utils::new_type::new_type;

/// Represents an authenticated user of the application.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(any(test, feature = "fake"), derive(fake::Dummy))]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub email: EmailAddress,

    /// The groups that a user is member of, thus has access to their respective
    /// projects.
    pub groups: Vec<Group>,
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
        use fake::Fake;
        use fake::Faker;

        use crate::common::domain::Group;

        Self {
            groups: [Group::new("admin"), Faker.fake()].into(),
            ..Faker.fake()
        }
    }
}

new_type![
    /// The name of a [`crate::domain::User`].
    #[derive(Deserialize, Serialize)]
    UserName(
        #[cfg_attr(any(test, feature = "fake"), dummy(faker = "fake::faker::name::en::Name()"))]
        String
    )
];

new_type![
    /// Represents an email address.
    ///
    /// > **Note:** This is not validated.
    #[derive(Deserialize, Serialize)]
    EmailAddress(
        #[cfg_attr(any(test, feature = "fake"), dummy(faker = "fake::faker::internet::en::FreeEmail()"))]
        String
    )
];

new_type![
    /// The unique id of a [`crate::domain::User`].
    #[derive(Deserialize, Serialize)]
    UserId(String)
];

#[cfg(test)]
mod user_tests {
    use fake::Fake;
    use fake::Faker;

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
