use serde::Deserialize;
use serde::Serialize;

use super::utils::new_type::new_type;

new_type![
    /// The name of a [`crate::domain::User`].
    #[derive(Deserialize, Serialize)]
    #[cfg_attr(test, derive(fake::Dummy))]
    UserName(
        #[cfg_attr(test, dummy(faker = "fake::faker::name::en::Name()"))]
        String
    )
];

new_type![
    /// Represents an email address.
    ///
    /// > **Note:** This is not validated.
    #[derive(Deserialize, Serialize)]
    #[cfg_attr(test, derive(fake::Dummy))]
    EmailAddress(
        #[cfg_attr(test, dummy(faker = "fake::faker::internet::en::FreeEmail()"))]
        String
    )
];

new_type![
    /// The unique id of a [`crate::domain::User`].
    #[derive(Deserialize, Serialize)]
    #[cfg_attr(test, derive(fake::Dummy))]
    UserId(String)
];
