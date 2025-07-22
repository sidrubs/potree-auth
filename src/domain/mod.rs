pub(crate) mod group;
pub(crate) mod project;
pub(crate) mod static_asset;
pub(crate) mod user;
pub(crate) mod utils;
pub(crate) mod value_objects;

use std::fmt;

pub(crate) use group::Group;
pub(crate) use static_asset::StaticAsset;
pub(crate) use user::User;

/// The various types of domain objects.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(fake::Dummy))]
pub enum ResourceType {
    Project,
    ProjectAsset,
    StaticAsset,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceType::Project => write!(f, "project"),
            ResourceType::ProjectAsset => write!(f, "project asset"),
            ResourceType::StaticAsset => write!(f, "static asset"),
        }
    }
}

impl From<&crate::services::authorization_engine::Resource<'_>> for ResourceType {
    fn from(value: &crate::services::authorization_engine::Resource) -> Self {
        match value {
            crate::services::authorization_engine::Resource::Project(_) => Self::Project,
        }
    }
}
