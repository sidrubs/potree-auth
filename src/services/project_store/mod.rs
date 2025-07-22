pub(crate) mod manifest_file;

use std::fmt::Debug;

use async_trait::async_trait;

use crate::domain::project::Project;
use crate::domain::value_objects::ProjectId;
use crate::error::ApplicationError;

/// Defines the functionality needed to for the application to interact with
/// persisted [`Project`]s.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProjectService: Debug + Send + Sync + 'static {
    /// Read a specific project by its `project_id`. The `project_id` is a
    /// unique identifier of the [`Project`].
    ///
    /// # Errors
    ///
    /// An [`ApplicationError::ResourceNotFound`] should be returned if the
    /// [`Project`] can't be found.
    async fn read(&self, project_id: &ProjectId) -> Result<Project, ApplicationError>;
}
