use super::action::Action;
use super::resource::ResourceIdentifier;
use super::resource::ResourceType;
use crate::user::domain::User;

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthorizationEngineError {
    #[error("{} is not authorized to {} the {:?}: {:?}", user.name, action, resource_type, resource_identifier)]
    NotAuthorized {
        user: Box<User>,
        action: Action,
        resource_identifier: Option<ResourceIdentifier>,
        resource_type: ResourceType,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,
}
