use std::path::Path;

use crate::{
    authorization::domain::resource::{
        Resource, ResourceIdentifier, ResourceInstance, ResourceType,
    },
    common::domain::project::Project,
};

/// A struct that is used to provide the required authZ data to the authorization engine.
#[derive(Debug)]
pub struct ProjectAssetResource<'a> {
    pub associated_project: &'a Project,
    pub asset_path: &'a Path,
}

impl Resource for ProjectAssetResource<'_> {
    fn resource_type(&self) -> crate::authorization::domain::resource::ResourceType {
        ResourceType::new("project_asset".to_owned())
    }
}

impl ResourceInstance for ProjectAssetResource<'_> {
    fn resource_identifier(&self) -> ResourceIdentifier {
        ResourceIdentifier::new(self.asset_path.to_string_lossy().to_string())
    }

    fn groups(&self) -> Vec<crate::common::domain::Group> {
        self.associated_project.groups.clone()
    }
}
