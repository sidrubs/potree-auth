//! AuthZ trait implementations for a [`Project`].

use super::Project;
use crate::authorization::domain::resource::Resource;
use crate::authorization::domain::resource::ResourceIdentifier;
use crate::authorization::domain::resource::ResourceInstance;
use crate::authorization::domain::resource::ResourceType;
use crate::common::domain::Group;

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
