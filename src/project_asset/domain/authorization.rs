use std::path::Path;

use crate::authorization::domain::resource::Resource;
use crate::authorization::domain::resource::ResourceIdentifier;
use crate::authorization::domain::resource::ResourceInstance;
use crate::authorization::domain::resource::ResourceType;
use crate::common::domain::Group;
use crate::common::domain::resource_type;
use crate::project::domain::Project;
use crate::user::domain::EmailAddress;

/// A struct that is used to provide the required authZ data to the
/// authorization engine.
#[derive(Debug)]
pub struct ProjectAssetResource<'a> {
    pub associated_project: &'a Project,
    pub asset_path: &'a Path,
}

impl Resource for ProjectAssetResource<'_> {
    fn resource_type(&self) -> crate::authorization::domain::resource::ResourceType {
        ResourceType::new(resource_type::PROJECT_ASSET.to_owned())
    }
}

impl ResourceInstance for ProjectAssetResource<'_> {
    fn resource_identifier(&self) -> ResourceIdentifier {
        ResourceIdentifier::new(self.asset_path.to_string_lossy().to_string())
    }

    fn groups(&self) -> Option<Vec<Group>> {
        Some(self.associated_project.groups.clone())
    }

    fn user_emails(&self) -> Option<Vec<EmailAddress>> {
        None
    }
}
