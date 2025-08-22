use super::group::Group;
use super::value_objects::ProjectId;
use super::value_objects::ProjectName;
use crate::authorization::domain::resource::Resource;
use crate::authorization::domain::resource::ResourceIdentifier;
use crate::authorization::domain::resource::ResourceInstance;
use crate::authorization::domain::resource::ResourceType;
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
    pub groups: Vec<Group>,
}

/// AuthZ trait implementations
impl Resource for Project {
    fn resource_type(&self) -> crate::authorization::domain::resource::ResourceType {
        ResourceType::new("project".to_owned())
    }
}

impl ResourceInstance for Project {
    fn resource_identifier(&self) -> crate::authorization::domain::resource::ResourceIdentifier {
        ResourceIdentifier::new(self.id.to_string())
    }

    fn groups(&self) -> Vec<Group> {
        self.groups.clone()
    }
}

/// Represents a project type for type-level (rather than instance-level) authZ.
#[derive(Debug)]
pub struct ProjectTypeResource;

impl Resource for ProjectTypeResource {
    fn resource_type(&self) -> crate::authorization::domain::resource::ResourceType {
        ResourceType::new("project".to_owned())
    }
}
