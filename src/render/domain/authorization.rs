use crate::authorization::domain::resource::Resource;
use crate::authorization::domain::resource::ResourceIdentifier;
use crate::authorization::domain::resource::ResourceInstance;
use crate::authorization::domain::resource::ResourceType;
use crate::common::domain::Group;
use crate::common::domain::resource_type;
use crate::project::domain::Project;
use crate::user::domain::EmailAddress;

/// A struct that is used to provide the required authZ data to the
/// authorization engine for a potree render instance.
#[derive(Debug)]
pub struct PotreeRenderResource<'a> {
    pub associated_project: &'a Project,
}

impl Resource for PotreeRenderResource<'_> {
    fn resource_type(&self) -> crate::authorization::domain::resource::ResourceType {
        ResourceType::new(resource_type::POTREE_RENDER.to_owned())
    }
}

impl ResourceInstance for PotreeRenderResource<'_> {
    fn resource_identifier(&self) -> ResourceIdentifier {
        ResourceIdentifier::new(format!("potree render: {}", self.associated_project.name))
    }

    fn groups(&self) -> Option<Vec<Group>> {
        Some(self.associated_project.groups.clone())
    }

    fn user_emails(&self) -> Option<Vec<EmailAddress>> {
        None
    }
}

/// A struct that is used to provide the required authZ data to the
/// authorization engine for the projects dashboard.
#[derive(Debug)]
pub struct ProjectDashboardResource;

impl Resource for ProjectDashboardResource {
    fn resource_type(&self) -> crate::authorization::domain::resource::ResourceType {
        ResourceType::new(resource_type::PROJECTS_DASHBOARD.to_owned())
    }
}
