use std::sync::Arc;

use web_route::ParameterizedRoute;
use web_route::WebRoute;

use super::super::domain::authorization::PotreeRenderResource;
use super::super::domain::authorization::ProjectDashboardResource;
use super::super::domain::not_found_render::NotFound;
use super::super::domain::potree_render::PotreeRender;
use super::super::domain::project_dashboard_render::ProjectDashboard;
use super::error::RenderingServiceError;
use crate::authorization::domain::action::Action;
use crate::authorization::ports::authorization_engine::AuthorizationEngine;
use crate::project::application::port::ProjectServicePort;
use crate::project::domain::ProjectId;
use crate::user::domain::User;

/// A service for rendering a project.
#[derive(Debug, Clone)]
pub struct RenderingService {
    /// Used to load project information.
    project_service: Arc<dyn ProjectServicePort>,

    /// Used to determine if a user ir authorized to view a project.
    authorization_engine: Arc<dyn AuthorizationEngine>,

    /// The route (parametrized) at which the project assets can be accessed.
    project_assets_route: ParameterizedRoute,

    /// The top-level route from which `potree` static assets are served.
    potree_assets_route: WebRoute,
}

impl RenderingService {
    pub fn new(
        project_service: Arc<dyn ProjectServicePort>,
        authorization_engine: Arc<dyn AuthorizationEngine>,
        project_assets_route: ParameterizedRoute,
        potree_assets_route: WebRoute,
    ) -> Self {
        Self {
            project_service,
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
        let project = self.project_service.read(user, project_id).await?;

        let potree_render_resource = PotreeRenderResource {
            associated_project: &project,
        };
        self.authorization_engine
            .can_on_instance(user, &Action::Read, &potree_render_resource)?;

        Ok(PotreeRender {
            project_title: project.name,
            potree_static_assets_path: self.potree_assets_route.clone(),
            potree_config_path: self.potree_config_route(project_id)?,
        })
    }

    /// Provides a dashboard showing all of the `user`'s projects.
    pub async fn project_dashboard(
        &self,
        user: &Option<User>,
        default_project_render_route: &ParameterizedRoute,
    ) -> Result<ProjectDashboard, RenderingServiceError> {
        self.authorization_engine
            .can_on_type(user, &Action::Read, &ProjectDashboardResource)?;

        let projects = self.project_service.list(user).await?;

        Ok(ProjectDashboard::from_domain_projects(
            projects,
            default_project_render_route,
        )?)
    }

    /// Provides a 404 page.
    pub async fn not_found(&self) -> Result<NotFound, RenderingServiceError> {
        Ok(NotFound)
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
    use crate::authorization::domain::action::Action;
    use crate::authorization::domain::error::AuthorizationEngineError;
    use crate::authorization::ports::authorization_engine::MockAuthorizationEngine;
    use crate::project::application::port::MockProjectServicePort;

    mod render_potree {

        use super::*;

        #[tokio::test]
        async fn should_return_the_correct_error_if_user_not_authenticated() {
            // Arrange
            let mut project_service = MockProjectServicePort::new();
            project_service.expect_read().return_const(Ok(Faker.fake()));
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can_on_instance()
                .return_const(Err(AuthorizationEngineError::NotAuthenticated));

            let rendering_service = RenderingService::new(
                Arc::new(project_service),
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
            let mut project_service = MockProjectServicePort::new();
            project_service.expect_read().return_const(Ok(Faker.fake()));
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can_on_instance()
                .return_const(Err(AuthorizationEngineError::NotAuthorized {
                    user: Faker.fake(),
                    action: Action::Read,
                    resource_identifier: Faker.fake(),
                    resource_type: Faker.fake(),
                }));

            let rendering_service = RenderingService::new(
                Arc::new(project_service),
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

    mod project_dashboard {

        use super::*;

        #[tokio::test]
        async fn should_return_the_correct_error_if_user_not_authenticated() {
            // Arrange
            let project_service = MockProjectServicePort::new();
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can_on_type()
                .return_const(Err(AuthorizationEngineError::NotAuthenticated));

            let rendering_service = RenderingService::new(
                Arc::new(project_service),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            let default_project_render_route =
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}");

            // Act
            let res = rendering_service
                .project_dashboard(&Faker.fake(), &default_project_render_route)
                .await;

            // Assert
            assert!(matches!(res, Err(RenderingServiceError::NotAuthenticated)));
        }

        #[tokio::test]
        async fn should_return_the_correct_error_if_user_not_authorized() {
            // Arrange
            let project_service = MockProjectServicePort::new();
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine.expect_can_on_type().return_const(Err(
                AuthorizationEngineError::NotAuthorized {
                    user: Faker.fake(),
                    action: Action::Read,
                    resource_identifier: Faker.fake(),
                    resource_type: Faker.fake(),
                },
            ));

            let rendering_service = RenderingService::new(
                Arc::new(project_service),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            let default_project_render_route =
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}");

            // Act
            let res = rendering_service
                .project_dashboard(&Faker.fake(), &default_project_render_route)
                .await;

            // Assert
            assert!(matches!(
                res,
                Err(RenderingServiceError::NotAuthorized { .. })
            ));
        }

        #[tokio::test]
        async fn should_return_the_correct_error_if_project_service_fails() {
            // Arrange
            let mut project_service = MockProjectServicePort::new();
            project_service.expect_list().return_const(Err(
                crate::project::application::error::ProjectServiceError::NotAuthenticated,
            ));
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can_on_type()
                .return_const(Ok(()));

            let rendering_service = RenderingService::new(
                Arc::new(project_service),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            let default_project_render_route =
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}");

            // Act
            let res = rendering_service
                .project_dashboard(&Faker.fake(), &default_project_render_route)
                .await;

            // Assert
            assert!(matches!(res, Err(RenderingServiceError::NotAuthenticated)));
        }

        #[tokio::test]
        async fn should_return_project_dashboard_when_successful() {
            // Arrange
            let dummy_projects: Vec<crate::project::domain::Project> =
                vec![Faker.fake(), Faker.fake()];
            let expected_project_count = dummy_projects.len();

            let mut project_service = MockProjectServicePort::new();
            project_service
                .expect_list()
                .return_const(Ok(dummy_projects));
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can_on_type()
                .return_const(Ok(()));

            let rendering_service = RenderingService::new(
                Arc::new(project_service),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            let default_project_render_route =
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}");

            // Act
            let res = rendering_service
                .project_dashboard(&Faker.fake(), &default_project_render_route)
                .await;

            // Assert
            assert!(res.is_ok());
            let dashboard = res.unwrap();
            assert_eq!(dashboard.projects.len(), expected_project_count);
        }

        #[tokio::test]
        async fn should_return_empty_project_dashboard_when_no_projects() {
            // Arrange
            let empty_projects: Vec<crate::project::domain::Project> = vec![];

            let mut project_service = MockProjectServicePort::new();
            project_service
                .expect_list()
                .return_const(Ok(empty_projects));
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can_on_type()
                .return_const(Ok(()));

            let rendering_service = RenderingService::new(
                Arc::new(project_service),
                Arc::new(authorization_engine),
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}/{*path}"),
                Faker.fake(),
            );

            let default_project_render_route =
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}");

            // Act
            let res = rendering_service
                .project_dashboard(&Faker.fake(), &default_project_render_route)
                .await;

            // Assert
            assert!(res.is_ok());
            let dashboard = res.unwrap();
            assert_eq!(dashboard.projects.len(), 0);
        }
    }
}
