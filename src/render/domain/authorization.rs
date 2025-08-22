use crate::{
    authorization::domain::resource::{
        Resource, ResourceIdentifier, ResourceInstance, ResourceType,
    },
    common::domain::project::Project,
};

/// A struct that is used to provide the required authZ data to the authorization engine for a potree render instance.
#[derive(Debug)]
pub struct PotreeRenderResource<'a> {
    pub associated_project: &'a Project,
}

impl Resource for PotreeRenderResource<'_> {
    fn resource_type(&self) -> crate::authorization::domain::resource::ResourceType {
        ResourceType::new("potree_render".to_owned())
    }
}

impl ResourceInstance for PotreeRenderResource<'_> {
    fn resource_identifier(&self) -> ResourceIdentifier {
        ResourceIdentifier::new(format!("potree render: {}", self.associated_project.name))
    }

    fn groups(&self) -> Vec<crate::common::domain::Group> {
        self.associated_project.groups.clone()
    }
}

/// A struct that is used to provide the required authZ data to the authorization engine for the projects dashboard.
#[derive(Debug)]
pub struct ProjectDashboardResource;

impl Resource for ProjectDashboardResource {
    fn resource_type(&self) -> crate::authorization::domain::resource::ResourceType {
        ResourceType::new("projects_dashboard".to_owned())
    }
}
