use std::sync::Arc;

use web_route::ParameterizedRoute;
use web_route::WebRoute;

use super::super::domain::potree_render::PotreeRender;
use super::error::RenderingServiceError;
use crate::common::domain::User;
use crate::common::domain::value_objects::ProjectId;
use crate::common::ports::authorization_engine::Action;
use crate::common::ports::authorization_engine::AuthorizationEngine;
use crate::common::ports::authorization_engine::Resource;
use crate::common::ports::project_datastore::ProjectDatastore;

/// A service for rendering a project.
#[derive(Debug, Clone)]
pub struct RenderingService {
    /// Used to load project information.
    project_datastore: Arc<dyn ProjectDatastore>,

    /// Used to determine if a user ir authorized to view a project.
    authorization_engine: Arc<dyn AuthorizationEngine>,

    /// The route (parametrized) at which the project assets can be accessed.
    project_assets_route: ParameterizedRoute,

    /// The top-level route from which `potree` static assets are served.
    potree_assets_route: WebRoute,
}

impl RenderingService {
    pub fn new(
        project_datastore: Arc<dyn ProjectDatastore>,
        authorization_engine: Arc<dyn AuthorizationEngine>,
        project_assets_route: ParameterizedRoute,
        potree_assets_route: WebRoute,
    ) -> Self {
        Self {
            project_datastore,
            authorization_engine,
            project_assets_route,
            potree_assets_route,
        }
    }

    /// Render a `potree` project.
    ///
    /// # Errors
    ///
    /// Will return an error if the project can not be found or the `user` is
    /// not authorized to view it.
    pub async fn render_potree(
        &self,
        user: &Option<User>,
        project_id: &ProjectId,
    ) -> Result<PotreeRender, RenderingServiceError> {
        let project = self.project_datastore.read(project_id).await?;

        self.authorization_engine
            .can(user, &Action::Read, &Resource::Project(&project))?;

        Ok(PotreeRender {
            project_title: project.name,
            potree_static_assets_path: self.potree_assets_route.clone(),
            potree_config_path: self.potree_config_route(project_id)?,
        })
    }

    /// Determine the route to the potree config file for a specific project
    /// (`project_id`).
    fn potree_config_route(
        &self,
        project_id: &ProjectId,
    ) -> Result<WebRoute, RenderingServiceError> {
        // This is copying the parameters from the `project_assets_server` project
        // assets route. It is possible for these to go out of date unless a common
        // import is used.
        //
        // Currently the only way to check that this works will be a top level
        // integration/e2e test.
        let params = serde_json::json!({
            "project_id": project_id,
            "path": "potree.json5",
        });

        Ok(self.project_assets_route.to_web_route(&params)?)
    }
}

#[cfg(test)]
mod project_rendering_service_tests {
    use std::sync::Arc;

    use fake::Fake;
    use fake::Faker;
    use web_route::ParameterizedRoute;
    use web_route::WebRoute;

    use super::super::super::application::error::RenderingServiceError;
    use super::super::super::application::service::RenderingService;
    use crate::common::ports::authorization_engine::AuthorizationEngineError;
    use crate::common::ports::authorization_engine::MockAuthorizationEngine;
    use crate::common::ports::project_datastore::MockProjectDatastore;

    mod render_potree {

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
                .expect_can()
                .return_const(Err(AuthorizationEngineError::NotAuthenticated));

            let project_asset_service = RenderingService::new(
                Arc::new(project_datastore),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            // Act
            let res = project_asset_service
                .render_potree(&Faker.fake(), &Faker.fake())
                .await;

            // Assert
            assert!(matches!(res, Err(RenderingServiceError::NotAuthenticated)));
        }
    }

    #[tokio::test]
    async fn should_return_the_correct_error_if_user_not_authorized() {
        // Arrange
        let mut project_datastore = MockProjectDatastore::new();
        project_datastore
            .expect_read()
            .return_const(Ok(Faker.fake()));
        let mut authorization_engine = MockAuthorizationEngine::new();
        authorization_engine.expect_can().return_const(Err(
            AuthorizationEngineError::NotAuthorized {
                user: Faker.fake(),
                resource_name: Faker.fake(),
                resource_type: Faker.fake(),
            },
        ));

        let project_asset_service = RenderingService::new(
            Arc::new(project_datastore),
            Arc::new(authorization_engine),
            ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
            Faker.fake(),
        );

        // Act
        let res = project_asset_service
            .render_potree(&Faker.fake(), &Faker.fake())
            .await;

        // Assert
        assert!(matches!(
            res,
            Err(RenderingServiceError::NotAuthorized { .. })
        ));
    }
}
