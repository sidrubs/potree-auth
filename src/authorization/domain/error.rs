use super::action::Action;
use super::resource::ResourceType;
use crate::common::domain::User;

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthorizationEngineError {
    #[error("{} is not authorized to {} the {:?}: {}", user.name, action, resource_type, resource_name)]
    NotAuthorized {
        user: Box<User>,
        action: Box<Action>,
        resource_name: String,
        resource_type: Box<ResourceType>,
    },

    #[error("user is not authenticated")]
    NotAuthenticated,
}
