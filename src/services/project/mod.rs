// pub(crate) mod manifest_file;

use async_trait::async_trait;
use std::fmt::Debug;

use crate::{domain::project::Project, error::ApplicationError};

/// Defines the functionality needed to for the application to interact with
/// [`Project`]s.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProjectService: Debug + Send + Sync + 'static {
    /// Read a specific project by its `project_name`. The `project_name` is a
    /// unique identifier of the [`Project`].
    ///
    /// # Errors
    ///
    /// An [`ApplicationError::ResourceNotFound`] should be returned if the
    /// [`Project`] can't be found.
    async fn read(&self, project_name: &str) -> Result<Project, ApplicationError>;
}
