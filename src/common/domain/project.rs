use std::collections::HashSet;

use super::group::Group;
use super::value_objects::ProjectId;
use super::value_objects::ProjectName;

/// Represents the metadata associated with a 3D model project.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct Project {
    pub id: ProjectId,

    /// A human readable name for the project.
    pub name: ProjectName,

    /// The groups that the project is a member of.
    pub groups: HashSet<Group>,
}
