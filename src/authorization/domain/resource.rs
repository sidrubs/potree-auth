use core::fmt;

use crate::common::domain::project::Project;

/// Defines a resource that can be accessed.
#[derive(Debug, Clone, PartialEq)]
pub enum Resource<'a> {
    /// A specific project (instance-level).
    Project(&'a Project),

    /// Projects in general (type-level). Usually for `list` actions.
    ProjectType,

    /// An asset associated with a specific project (instance-level).
    ProjectAsset(&'a Project),

    /// Potree rendering for a specific project (instance-level).
    PotreeRender(&'a Project),

    /// The dashboard that lists all of a user's projects (type-level).
    ProjectDashboard,
}

/// The various types of domain objects.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(fake::Dummy))]
pub enum ResourceType {
    Project,
    ProjectAsset,
    PotreeRender,
    ProjectDashboard,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceType::Project => write!(f, "project"),
            ResourceType::ProjectAsset => write!(f, "project asset"),
            ResourceType::PotreeRender => write!(f, "potree render"),
            ResourceType::ProjectDashboard => write!(f, "project dashboard"),
        }
    }
}
