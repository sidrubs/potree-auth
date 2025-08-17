use std::collections::HashSet;

use super::group::Group;
use super::value_objects::ProjectId;
use super::value_objects::ProjectName;
use crate::common::domain::value_objects::ProjectDescription;

/// Represents the metadata associated with a 3D model project.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct Project {
    pub id: ProjectId,

    /// A human readable name for the project.
    pub name: ProjectName,

    /// Optional additional context about the project.
    pub description: Option<ProjectDescription>,

    /// The groups that the project is a member of.
    pub groups: HashSet<Group>,
}
