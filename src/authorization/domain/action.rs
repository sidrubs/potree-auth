use super::resource::Resource;
use super::resource::ResourceType;

/// Defines actions that can be performed on a [`Resource`].
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Read,
    List,
    Write,
    Update,
    Delete,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Read => write!(f, "read"),
            Action::List => write!(f, "list"),
            Action::Write => write!(f, "write"),
            Action::Update => write!(f, "update"),
            Action::Delete => write!(f, "delete"),
        }
    }
}

impl From<&Resource<'_>> for ResourceType {
    fn from(value: &Resource) -> Self {
        match value {
            Resource::Project(_) | Resource::ProjectType => Self::Project,
            Resource::ProjectAsset(_) => Self::ProjectAsset,
            Resource::PotreeRender(_) => Self::PotreeRender,
            Resource::ProjectDashboard => Self::ProjectDashboard,
        }
    }
}
