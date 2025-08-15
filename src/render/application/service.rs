use std::sync::Arc;

use web_route::ParameterizedRoute;
use web_route::WebRoute;

use super::super::domain::potree_render::PotreeRender;
use super::error::RenderingServiceError;
use crate::common::domain::User;
use crate::common::domain::project::Project;
use crate::common::domain::value_objects::ProjectId;
use crate::common::ports::authorization_engine::Action;
use crate::common::ports::authorization_engine::AuthorizationEngine;
use crate::common::ports::authorization_engine::Resource;
use crate::common::ports::project_repository::ProjectRepository;

use crate::render::domain::project_dashboard_render::ProjectDashboard;

/// A service for rendering a project.
#[derive(Debug, Clone)]
pub struct RenderingService {
    /// Used to load project information.
    project_repository: Arc<dyn ProjectRepository>,

    /// Used to determine if a user ir authorized to view a project.
    authorization_engine: Arc<dyn AuthorizationEngine>,

    /// The route (parametrized) at which the project assets can be accessed.
    project_assets_route: ParameterizedRoute,

    /// The top-level route from which `potree` static assets are served.
    potree_assets_route: WebRoute,
}

impl RenderingService {
    pub fn new(
        project_datastore: Arc<dyn ProjectRepository>,
        authorization_engine: Arc<dyn AuthorizationEngine>,
        project_assets_route: ParameterizedRoute,
        potree_assets_route: WebRoute,
    ) -> Self {
        Self {
            project_repository: project_datastore,
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
        let project = self.project_repository.read(project_id).await?;

        self.authorization_engine
            .can(user, &Action::Read, &Resource::PotreeRender(&project))?;

        Ok(PotreeRender {
            project_title: project.name,
            potree_static_assets_path: self.potree_assets_route.clone(),
            potree_config_path: self.potree_config_route(project_id)?,
        })
    }

    /// PUT IN DOCSTRING
    ///
    /// # Todo
    ///
    /// - Project interactions should really be its own service. Doesn't need `http` for now.
    pub async fn project_dashboard(
        &self,
        user: &Option<User>,
    ) -> Result<ProjectDashboard, RenderingServiceError> {
        let projects = self.list_allowed_projects(user).await?;

        Ok(ProjectDashboard { projects })
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

    /// List all of the projects a `user` is allowed to read.
    ///
    /// # Todo
    ///
    /// - Unit tests for the filtering out of not allowed projects.
    async fn list_allowed_projects(
        &self,
        user: &Option<User>,
    ) -> Result<Vec<Project>, RenderingServiceError> {
        let projects = self.project_repository.list().await?;

        // Filter out the projects that the user is not allowed to read.
        // TODO: Need to work out authentication. Shouldn't be handled by the authZ engine.
        let allowed_projects = projects
            .into_iter()
            .filter(|p| {
                self.authorization_engine
                    .can(user, &Action::Read, &Resource::Project(&p))
                    .is_ok()
            })
            .collect();

        Ok(allowed_projects)
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
    use crate::common::ports::project_repository::MockProjectRepository;

    mod render_potree {
        use super::*;

        #[tokio::test]
        async fn should_return_the_correct_error_if_user_not_authenticated() {
            // Arrange
            let mut project_datastore = MockProjectRepository::new();
            project_datastore
                .expect_read()
                .return_const(Ok(Faker.fake()));
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can()
                .return_const(Err(AuthorizationEngineError::NotAuthenticated));

            let rendering_service = RenderingService::new(
                Arc::new(project_datastore),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            // Act
            let res = rendering_service
                .render_potree(&Faker.fake(), &Faker.fake())
                .await;

            // Assert
            assert!(matches!(res, Err(RenderingServiceError::NotAuthenticated)));
        }

        #[tokio::test]
        async fn should_return_the_correct_error_if_user_not_authorized() {
            // Arrange
            let mut project_datastore = MockProjectRepository::new();
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

            let rendering_service = RenderingService::new(
                Arc::new(project_datastore),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            // Act
            let res = rendering_service
                .render_potree(&Faker.fake(), &Faker.fake())
                .await;

            // Assert
            assert!(matches!(
                res,
                Err(RenderingServiceError::NotAuthorized { .. })
            ));
        }
    }

    mod list_allowed_projects {
        use std::sync::Mutex;

        use super::*;
        use crate::common::domain::project::Project;

        #[tokio::test]
        async fn should_return_the_projects_that_the_user_is_allowed_to_read() {
            // Arrange
            let dummy_projects = Faker.fake::<Vec<Project>>();

            let mut project_datastore = MockProjectRepository::new();
            project_datastore
                .expect_list()
                .return_const(Ok(dummy_projects.clone()));
            let mut authorization_engine = MockAuthorizationEngine::new();
            let authorization_engine_call_count = Arc::new(Mutex::new(0));
            let authorization_engine_call_count_clone =
                Arc::clone(&authorization_engine_call_count);
            authorization_engine.expect_can().returning(move |_, _, _| {
                let mut count = authorization_engine_call_count_clone.lock().unwrap();
                let res = if *count % 2 == 0 {
                    Err(AuthorizationEngineError::NotAuthorized {
                        user: Faker.fake(),
                        resource_name: Faker.fake(),
                        resource_type: Faker.fake(),
                    })
                } else {
                    Ok(())
                };
                *count += 1;
                res
            });

            let rendering_service = RenderingService::new(
                Arc::new(project_datastore),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            // Act
            let allowed_projects = rendering_service
                .list_allowed_projects(&Faker.fake())
                .await
                .unwrap();

            // Assert
            assert_eq!(allowed_projects.len(), dummy_projects.len() / 2);
        }
    }
}
