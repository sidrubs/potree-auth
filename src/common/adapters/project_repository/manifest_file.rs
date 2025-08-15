use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

use crate::common::domain::group::Group;
use crate::common::domain::project::Project;
use crate::common::domain::value_objects::ProjectId;
use crate::common::domain::value_objects::ProjectName;
use crate::common::ports::project_repository::ProjectRepository;
use crate::common::ports::project_repository::ProjectRepositoryError;

/// The name of the project manifest files.
const MANIFEST_FILE_NAME: &str = "manifest.yml";

/// A manifest file backed implementation of the [`ProjectRepository`] trait.
///
/// Expects the `projects_directory` to be the parent directory to a collection
/// of project directories, each representing a project. The name of the
/// directory is the name of the project (it is thus inherently a unique
/// identifier). To be considered a valid project, the subdirectory needs to
/// contain a `manifest.yml` file that can be deserialized to a
/// [`ProjectManifest`] struct.
#[derive(Debug, Clone)]
pub struct ManifestFileProjectRepository {
    /// The directory containing all the projects.
    projects_directory: PathBuf,
}

impl ManifestFileProjectRepository {
    /// Creates a new [`ManifestFileProjectService`] instance with the specified
    /// `projects_directory`.
    pub fn new<P: AsRef<Path>>(projects_directory: P) -> Self {
        Self {
            projects_directory: projects_directory.as_ref().to_path_buf(),
        }
    }

    #[tracing::instrument]
    async fn read(&self, project_id: ProjectId) -> Result<Project, ProjectRepositoryError> {
        let project_manifest_path = self
            .projects_directory
            .join(String::from(project_id.clone()))
            .join(MANIFEST_FILE_NAME);

        let manifest_bytes = tokio::fs::read(&project_manifest_path)
            .await
            .map_err(|_e| ProjectRepositoryError::ResourceNotFound {
                id: project_id.clone(),
            })?;

        let manifest = serde_yml::from_slice::<ProjectManifest>(&manifest_bytes).map_err(|_e| {
            ProjectRepositoryError::Parsing {
                id: project_id.clone(),
            }
        })?;

        Ok(manifest.into_project(&project_id))
    }

    #[tracing::instrument]
    async fn list(&self) -> Result<Vec<Project>, ProjectRepositoryError> {
        let mut dir_contents = tokio::fs::read_dir(&self.projects_directory)
            .await
            .map_err(|_e| ProjectRepositoryError::Infrastucture {
                message: format!(
                    "unable to read from the directory: {}",
                    self.projects_directory.to_string_lossy()
                ),
            })?;

        // Find all top-level directories, they represent the project ids.
        let mut project_ids = Vec::new();
        while let Some(entry) =
            dir_contents
                .next_entry()
                .await
                .map_err(|_e| ProjectRepositoryError::Infrastucture {
                    message: format!(
                        "unable to read from the directory: {}",
                        self.projects_directory.to_string_lossy()
                    ),
                })?
        {
            let file_type =
                entry
                    .file_type()
                    .await
                    .map_err(|_e| ProjectRepositoryError::Infrastucture {
                        message: format!(
                            "unable to read from the directory: {}",
                            self.projects_directory.to_string_lossy()
                        ),
                    })?;
            if file_type.is_dir() {
                project_ids.push(entry.file_name());
            }
        }

        // Asynchronously read the projects for each project id.
        let loaded_projects = futures::future::join_all(project_ids.iter().map(|id| {
            let id = ProjectId::new(id.to_string_lossy().to_string());
            self.read(id)
        }))
        .await;

        // Filter out the the projects that did not load successfully (e.g. invalid
        // manifest file).
        loaded_projects
            .into_iter()
            .filter(|res| res.is_ok())
            .collect::<Result<Vec<_>, _>>()
    }
}

#[async_trait]
impl ProjectRepository for ManifestFileProjectRepository {
    async fn read(&self, project_id: &ProjectId) -> Result<Project, ProjectRepositoryError> {
        Self::read(self, project_id.clone()).await
    }

    async fn list(&self) -> Result<Vec<Project>, ProjectRepositoryError> {
        Self::list(self).await
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
    use fake::Fake;
    use fake::Faker;

    use super::*;

    /// The name of the project manifest files for testing purposes (so tests
    /// can detect a change in filename).
    const TEST_MANIFEST_FILE_NAME: &str = "manifest.yml";

    /// Writes the `project` to `projects_dir` as a manifest file in the correct
    /// subdirectory.
    fn write_to_project_manifest<P: AsRef<Path>>(project: &Project, projects_dir: P) {
        let project_dir = PathBuf::new().join(&projects_dir).join(project.id.as_str());

        std::fs::create_dir(&project_dir).unwrap();

        let manifest_file = ProjectManifest::from_project(project);

        std::fs::write(
            project_dir.join(TEST_MANIFEST_FILE_NAME),
            serde_yml::to_string(&manifest_file).unwrap(),
        )
        .unwrap();
    }

    /// Creates an empty dir `dir_name` in the `projects_dir`.
    fn create_empty_project_dir<P: AsRef<Path>>(dir_name: &str, projects_dir: P) {
        let project_dir = PathBuf::new().join(&projects_dir).join(dir_name);

        std::fs::create_dir(&project_dir).unwrap();
    }

    mod read {
        use super::*;

        #[tokio::test]
        async fn should_read_the_correct_project() {
            // Arrange
            let project = Faker.fake::<Project>();
            let diversion_project = Faker.fake::<Project>();

            let projects_dir = tempfile::tempdir().unwrap();

            write_to_project_manifest(&project, &projects_dir);
            write_to_project_manifest(&diversion_project, &projects_dir);

            let service = ManifestFileProjectRepository::new(&projects_dir);

            // Act
            let recovered_project = service.read(project.id.clone()).await.unwrap();

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

            let service = ManifestFileProjectRepository::new(&projects_dir);

            // Act
            let res = service.read(non_existent_project.id.clone()).await;

            // Assert
            assert!(
                matches!(res, Err(ProjectRepositoryError::ResourceNotFound { id }) if id == non_existent_project.id)
            );
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
                serde_yml::to_string(&invalid_manifest_file).unwrap(),
            )
            .unwrap();

            let service = ManifestFileProjectRepository::new(&projects_dir);

            // Act
            let res = service.read(project_id.clone()).await;

            // Assert
            assert!(matches!(res, Err(ProjectRepositoryError::Parsing { id }) if id == project_id));
        }
    }

    mod list {
        use super::*;

        #[tokio::test]
        async fn should_list_all_available_valid_projects() {
            // Arrange
            let mut projects = fake::vec![Project; 1..40];
            let projects_dir = tempfile::tempdir().unwrap();

            projects
                .iter()
                .for_each(|project| write_to_project_manifest(project, &projects_dir));

            create_empty_project_dir("invalid-project-missing-manifest", &projects_dir); // invalid project should not create an error on listing projects.

            let service = ManifestFileProjectRepository::new(&projects_dir);

            // Act
            let mut recovered_projects = service.list().await.unwrap();

            // Assert
            projects.sort_by_key(|p| p.id.clone());
            recovered_projects.sort_by_key(|p| p.id.clone());
            assert_eq!(recovered_projects, projects);
        }

        #[tokio::test]
        async fn should_return_an_empty_vec_if_no_projects_in_dir() {
            // Arrange
            let projects_dir = tempfile::tempdir().unwrap();

            let service = ManifestFileProjectRepository::new(&projects_dir);

            // Act
            let recovered_projects = service.list().await.unwrap();

            // Assert
            assert!(recovered_projects.is_empty());
        }

        #[tokio::test]
        async fn should_return_correct_err_if_non_existent_project_dir() {
            // Arrange
            let projects_dir = "/does/not/exist";

            let service = ManifestFileProjectRepository::new(projects_dir);

            // Act
            let res = service.list().await;

            // Assert
            assert!(matches!(
                res,
                Err(ProjectRepositoryError::Infrastucture { .. })
            ));
        }
    }
}
