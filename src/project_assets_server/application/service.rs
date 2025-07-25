use std::path::Path;
use std::sync::Arc;

use http::HeaderMap;

use super::super::ports::project_asset_store::ProjectAssetStore;
use super::super::ports::project_datastore::ProjectDatastore;
use super::error::ProjectAssetsServiceError;
use crate::common::domain::StaticAsset;
use crate::common::domain::User;
use crate::common::domain::value_objects::ProjectId;
use crate::common::ports::authorization_engine::Action;
use crate::common::ports::authorization_engine::AuthorizationEngine;
use crate::common::ports::authorization_engine::Resource;

/// A service for interacting with project assets.
pub struct ProjectAssetService {
    project_datastore: Arc<dyn ProjectDatastore>,
    project_asset_store: Arc<dyn ProjectAssetStore>,
    authorization_engine: Arc<dyn AuthorizationEngine>,
}

impl ProjectAssetService {
    pub fn new(
        project_datastore: Arc<dyn ProjectDatastore>,
        project_asset_store: Arc<dyn ProjectAssetStore>,
        authorization_engine: Arc<dyn AuthorizationEngine>,
    ) -> Self {
        Self {
            project_datastore,
            project_asset_store,
            authorization_engine,
        }
    }

    /// Read a specific project asset. Optional `request_headers` can be
    /// provided to specify various instructions as to how to read and format
    /// the resulting data.
    pub async fn request_asset(
        &self,
        user: &Option<User>,
        project_id: &ProjectId,
        asset_path: &Path,
        request_headers: Option<HeaderMap>,
    ) -> Result<StaticAsset, ProjectAssetsServiceError> {
        let project = self.project_datastore.read(project_id).await?;
        self.authorization_engine.assert_allowed(
            user,
            &Resource::Project(&project),
            &Action::Read,
        )?;

        // Build a path to the asset. The asset would be within its project directory.
        let asset_path = Path::new(project_id.as_str()).join(asset_path);

        Ok(self
            .project_asset_store
            .get_asset(&asset_path, request_headers)
            .await?)
    }
}

#[cfg(test)]
mod project_asset_service_tests {
    use fake::Fake;
    use fake::Faker;

    use super::super::super::ports::project_asset_store::MockProjectAssetStore;
    use super::super::super::ports::project_datastore::MockProjectDatastore;
    use super::*;
    use crate::common::domain::project::Project;
    use crate::common::ports::authorization_engine::AuthorizationEngineError;
    use crate::common::ports::authorization_engine::MockAuthorizationEngine;
    use crate::common::ports::authorization_engine::ResourceType;

    mod request_asset {

        use super::*;

        #[tokio::test]
        async fn should_return_the_correct_error_if_user_not_authenticated() {
            // Arrange
            let mut project_datastore = MockProjectDatastore::new();
            project_datastore
                .expect_read()
                .return_const(Ok(Faker.fake()));
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_assert_allowed()
                .return_const(Err(AuthorizationEngineError::NotAuthenticated));
            let project_asset_store = MockProjectAssetStore::new();

            let project_asset_service = ProjectAssetService::new(
                Arc::new(project_datastore),
                Arc::new(project_asset_store),
                Arc::new(authorization_engine),
            );

            // Act
            let res = project_asset_service
                .request_asset(&Faker.fake(), &Faker.fake(), &Path::new(""), Faker.fake())
                .await;

            // Assert
            assert!(matches!(
                res,
                Err(ProjectAssetsServiceError::NotAuthenticated)
            ))
        }
    }

    #[tokio::test]
    async fn should_return_the_correct_error_if_user_not_authorized() {
        // Arrange
        let dummy_user = Faker.fake::<User>();
        let dummy_resource_name = Faker.fake::<Project>().name;

        let mut project_datastore = MockProjectDatastore::new();
        project_datastore
            .expect_read()
            .return_const(Ok(Faker.fake()));
        let mut authorization_engine = MockAuthorizationEngine::new();
        authorization_engine
            .expect_assert_allowed()
            .return_const(Err(AuthorizationEngineError::NotAuthorized {
                user: dummy_user.clone(),
                resource_name: dummy_resource_name.to_string(),
                resource_type: ResourceType::Project,
            }));
        let project_asset_store = MockProjectAssetStore::new();

        let project_asset_service = ProjectAssetService::new(
            Arc::new(project_datastore),
            Arc::new(project_asset_store),
            Arc::new(authorization_engine),
        );

        // Act
        let res = project_asset_service
            .request_asset(&Faker.fake(), &Faker.fake(), &Path::new(""), Faker.fake())
            .await;

        // Assert
        assert!(matches!(
            res,
            Err(ProjectAssetsServiceError::NotAuthorized { .. })
        ))
    }
}
