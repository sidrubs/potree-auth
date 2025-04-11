use std::collections::HashSet;

use super::group::Group;

/// Represents the metadata associated with a 3D model project.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct Project {
    pub name: String,

    /// The groups that the project is a member of.
    pub groups: HashSet<Group>,
}
