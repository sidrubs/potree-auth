use std::fmt::Debug;

use async_trait::async_trait;

use super::super::domain::Project;
use super::super::domain::ProjectId;
use super::error::ProjectServiceError;
use crate::user::domain::User;

/// Defines the functionality provided by the
/// [`super::service::ProjectService`] so that it can be
/// mocked for consuming services.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProjectServicePort: Debug + Send + Sync + 'static {
    // Read a specific project (`project_id`). `user` is used for
    /// authorization.
    async fn read(
        &self,
        user: &Option<User>,
        project_id: &ProjectId,
    ) -> Result<Project, ProjectServiceError>;

    /// List the projects that a user is allowed to view.
    async fn list(&self, user: &Option<User>) -> Result<Vec<Project>, ProjectServiceError>;
}
