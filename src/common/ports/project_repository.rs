use std::fmt::Debug;

use async_trait::async_trait;

use crate::common::domain::project::Project;
use crate::common::domain::value_objects::ProjectId;

/// Defines the functionality needed to for the application to interact with
/// persisted [`Project`]s.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProjectRepository: Debug + Send + Sync + 'static {
    /// Read a specific project by its `project_id`. The `project_id` is a
    /// unique identifier of the [`Project`].
    ///
    /// # Errors
    ///
    /// - [`ProjectRepositoryError::ResourceNotFound`] should be returned if the
    /// [`Project`] can't be found.
    /// - [`ProjectRepositoryError::Parsing`] if the project has an invalid
    ///   format.
    async fn read(&self, project_id: &ProjectId) -> Result<Project, ProjectRepositoryError>;

    /// List all the projects available in the datastore.
    ///
    /// # Errors
    ///
    /// - [`ProjectRepositoryError::Parsing`] if the project has an invalid
    ///   format.
    async fn list(&self) -> Result<Vec<Project>, ProjectRepositoryError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProjectRepositoryError {
    #[error("the `Project` ({id}) could not be found")]
    ResourceNotFound { id: ProjectId },

    #[error("unable to parse the `Project` ({id}) ")]
    Parsing { id: ProjectId },

    #[error("unable to interact with the datastore backend: {message}")]
    Infrastucture { message: String },
}
