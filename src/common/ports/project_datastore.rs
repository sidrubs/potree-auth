use std::fmt::Debug;

use async_trait::async_trait;

use crate::common::domain::project::Project;
use crate::common::domain::value_objects::ProjectId;

/// Defines the functionality needed to for the application to interact with
/// persisted [`Project`]s.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProjectDatastore: Debug + Send + Sync + 'static {
    /// Read a specific project by its `project_id`. The `project_id` is a
    /// unique identifier of the [`Project`].
    ///
    /// # Errors
    ///
    /// An [`ProjectDatastoreError::ResourceNotFound`] should be returned if the
    /// [`Project`] can't be found.
    async fn read(&self, project_id: &ProjectId) -> Result<Project, ProjectDatastoreError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProjectDatastoreError {
    #[error("the `Project` ({id}) could not be found")]
    ResourceNotFound { id: ProjectId },

    #[error("unable to parse the `Project` ({id}) ")]
    Parsing { id: ProjectId },
}
