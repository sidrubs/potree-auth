use web_route::WebRoute;

use crate::application::routes::PROJECT_ASSETS;
use crate::application::routes::ProjectAssetParams;
use crate::domain::value_objects::ProjectId;

/// Path to the `potree` config file for a specific project.
pub fn potree_config_path(project_id: &ProjectId) -> WebRoute {
    let params = ProjectAssetParams {
        project_id: project_id.clone(),
        path: WebRoute::new("potree.json5"),
    };

    PROJECT_ASSETS.to_web_route(&params).unwrap()
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
            WebRoute::new("/project-assets/test_project/potree.json5")
        );
    }
}
