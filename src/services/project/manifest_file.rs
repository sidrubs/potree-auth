use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        ResourceType,
        group::Group,
        project::Project,
        value_objects::{ProjectId, ProjectName},
    },
    error::ApplicationError,
};

use super::ProjectService;

/// The name of the project manifest files.
const MANIFEST_FILE_NAME: &str = "manifest.json";

/// A manifest file backed implementation of the [`ProjectService`] trait.
///
/// Expects the `projects_directory` to be the parent directory to a collection
/// of project directories, each representing a project. The name of the
/// directory is the name of the project (it is thus inherently a unique
/// identifier). To be considered a valid project, the subdirectory needs to
/// contain a `manifest.json` file that can be deserialized to a
/// [`ProjectManifest`] struct.
#[derive(Debug, Clone)]
pub(crate) struct ManifestFileProjectService {
    /// The directory containing all the projects.
    projects_directory: PathBuf,
}

impl ManifestFileProjectService {
    /// Creates a new [`ManifestFileProjectService`] instance with the specified
    /// `projects_directory`.
    pub fn new<P: AsRef<Path>>(projects_directory: P) -> Self {
        Self {
            projects_directory: projects_directory.as_ref().to_path_buf(),
        }
    }

    #[tracing::instrument]
    async fn read(&self, project_id: &ProjectId) -> Result<Project, ApplicationError> {
        let project_manifest_path = self
            .projects_directory
            .join(String::from(project_id.clone()))
            .join(MANIFEST_FILE_NAME);

        let manifest_bytes = tokio::fs::read(&project_manifest_path)
            .await
            .map_err(|_err| ApplicationError::ResourceNotFound {
                resource_name: project_id.to_string(),
                resource_type: ResourceType::Project,
            })?;

        let manifest =
            serde_json::from_slice::<ProjectManifest>(&manifest_bytes).map_err(|err| {
                ApplicationError::ServerError(format!(
                    "unable to parse {:?}: {}",
                    project_manifest_path, err
                ))
            })?;

        Ok(manifest.into_project(project_id))
    }
}

#[async_trait]
impl ProjectService for ManifestFileProjectService {
    async fn read(&self, project_id: &ProjectId) -> Result<Project, ApplicationError> {
        Self::read(self, project_id).await
    }
}

/// Represents the contents of a `manifest.json` file that is stored in a
/// project directory.
#[derive(Debug, Clone, Deserialize, Serialize)]
struct ProjectManifest {
    pub name: ProjectName,

    /// The groups that the project is a member of.
    pub groups: HashSet<Group>,
}

impl ProjectManifest {
    /// Converts a [`ProjectManifest`] into a [`Project`]. The `project_id`
    /// represent the unique identifying slug of the project.
    pub fn into_project(self, project_id: &ProjectId) -> Project {
        let Self { groups, name } = self;

        Project {
            id: project_id.clone(),
            name,
            groups,
        }
    }
}

#[cfg(test)]
impl ProjectManifest {
    pub fn from_project(project: &Project) -> Self {
        let Project { name, groups, .. } = project.clone();

        Self { name, groups }
    }
}

#[cfg(test)]
mod manifest_file_project_service_tests {
    use fake::{Fake, Faker};

    use super::*;

    /// The name of the project manifest files for testing purposes (so tests
    /// can detect a change in filename).
    const TEST_MANIFEST_FILE_NAME: &str = "manifest.json";

    /// Writes the `project` to `projects_dir` as a manifest file in the correct
    /// subdirectory.
    fn write_to_project_manifest<P: AsRef<Path>>(project: &Project, projects_dir: P) {
        let project_dir = PathBuf::new().join(&projects_dir).join(project.id.as_str());

        std::fs::create_dir(&project_dir).unwrap();

        let manifest_file = ProjectManifest::from_project(project);

        std::fs::write(
            project_dir.join(TEST_MANIFEST_FILE_NAME),
            serde_json::to_vec(&manifest_file).unwrap(),
        )
        .unwrap();
    }

    #[tokio::test]
    async fn should_read_the_correct_project() {
        // Arrange
        let project = Faker.fake::<Project>();
        let diversion_project = Faker.fake::<Project>();

        let projects_dir = tempfile::tempdir().unwrap();

        write_to_project_manifest(&project, &projects_dir);
        write_to_project_manifest(&diversion_project, &projects_dir);

        let service = ManifestFileProjectService::new(&projects_dir);

        // Act
        let recovered_project = service.read(&project.id).await.unwrap();

        // Assert
        assert_eq!(recovered_project, project);
    }

    #[tokio::test]
    async fn should_return_an_error_if_unable_to_find_project() {
        // Arrange
        let non_existent_project = Faker.fake::<Project>();
        let diversion_project = Faker.fake::<Project>();

        let projects_dir = tempfile::tempdir().unwrap();

        write_to_project_manifest(&diversion_project, &projects_dir);

        let service = ManifestFileProjectService::new(&projects_dir);

        // Act
        let res = service.read(&non_existent_project.id).await;

        // Assert
        assert!(
            matches!(res, Err(ApplicationError::ResourceNotFound { resource_name, resource_type }) if resource_name == non_existent_project.id.as_ref() && resource_type == ResourceType::Project)
        )
    }

    #[tokio::test]
    async fn should_return_an_error_if_unable_to_deserialize_project() {
        // Arrange
        let projects_dir = tempfile::tempdir().unwrap();
        let project_id = Faker.fake::<ProjectId>();

        let project_dir = PathBuf::new().join(&projects_dir).join(project_id.as_str());

        std::fs::create_dir(&project_dir).unwrap();

        let invalid_manifest_file = Faker.fake::<String>();

        std::fs::write(
            project_dir.join(TEST_MANIFEST_FILE_NAME),
            serde_json::to_vec(&invalid_manifest_file).unwrap(),
        )
        .unwrap();

        let service = ManifestFileProjectService::new(&projects_dir);

        // Act
        let res = service.read(&project_id).await;

        // Assert
        assert!(matches!(res, Err(ApplicationError::ServerError(_))))
    }
}
