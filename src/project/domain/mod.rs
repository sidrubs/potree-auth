pub mod authorization;

use crate::common::domain::Group;
use crate::common::domain::utils::new_type::new_type;

/// Represents the metadata associated with a 3D model project.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(any(test, feature = "fake"), derive(fake::Dummy))]
pub struct Project {
    pub id: ProjectId,

    /// A human readable name for the project.
    pub name: ProjectName,

    /// Optional additional context about the project.
    pub description: Option<ProjectDescription>,

    /// The groups that the project is a member of.
    pub groups: Vec<Group>,
}

new_type![
    /// The unique identifying slug of a [`Project`].
    #[derive(serde::Deserialize, serde::Serialize)]
    ProjectId(String)
];

new_type![
    /// The name of a [`Project`].
    #[derive(serde::Deserialize, serde::Serialize)]
    ProjectName(
        #[cfg_attr(test, dummy(faker = "fake::faker::name::en::Name()"))]
        String
    )
];

new_type![
    /// A high-level description of a [`Project`]. Gives the user more context.
    #[derive(serde::Deserialize, serde::Serialize)]
    ProjectDescription(String)
];
