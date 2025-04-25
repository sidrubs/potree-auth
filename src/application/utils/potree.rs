use std::path::PathBuf;

use crate::{
    application::routes::{PROJECT_ASSETS, PROJECT_ROOT},
    domain::value_objects::ProjectId,
};

/// Path to the `potree` config file for a specific project.
pub fn potree_config_path(project_id: &ProjectId) -> PathBuf {
    PathBuf::new()
        .join(PROJECT_ROOT)
        .join(project_id.as_str())
        .join(PROJECT_ASSETS)
        .join("potree.json5")
}

#[cfg(test)]
mod potree_config_path {
    use super::*;

    #[test]
    fn should_return_the_correct_path() {
        // Act
        let config_path = potree_config_path(&ProjectId::new("test_project".to_owned()));

        // Assert
        assert_eq!(
            config_path,
            PathBuf::from("/project/test_project/assets/potree.json5")
        );
    }
}
