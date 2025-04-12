use std::{collections::HashSet, path::PathBuf};

use crate::domain::{group::Group, project::Project};

/// A manifest file backed implementation of the [`ProjectService`] trait.
///
/// Expects the `projects_directory` to be the parent directory to a collection
/// of project directories, each representing a project. The name of the
/// directory is the name of the project (it is thus inherently a unique
/// identifier). To be considered a valid project, the subdirectory needs to
/// contain a `manifest.json` file that can be deserialized to a
/// [`ProjectManifest`] struct.
pub(crate) struct ManifestFileProjectService {
    /// The directory containing all the projects.
    projects_directory: PathBuf,
}

/// Represents the contents of a `manifest.json` file that is stored in a
/// project directory.
#[derive(Debug, Clone, serde::Deserialize)]
struct ProjectManifest {
    /// The groups that the project is a member of.
    pub groups: HashSet<Group>,
}

impl ProjectManifest {
    /// Converts a [`ProjectManifest`] into a [`Project`]. The `project_id`
    /// represent the unique identifying slug of the project.
    pub fn into_project(self, project_id: &str) -> Project {
        let Self { groups } = self;

        Project {
            id: project_id.to_owned(),
            groups,
        }
    }
}
