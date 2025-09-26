use std::sync::Arc;

use async_trait::async_trait;

use super::super::domain::Project;
use super::super::domain::ProjectId;
use super::super::ports::project_repository::ProjectRepository;
use super::error::ProjectServiceError;
use crate::authorization::domain::action::Action;
use crate::authorization::ports::authorization_engine::AuthorizationEngine;
use crate::project::application::port::ProjectServicePort;
use crate::project::domain::authorization::ProjectTypeResource;
use crate::user::domain::User;

/// A service for interacting with projects.
#[derive(Debug, Clone)]
pub struct ProjectService {
    project_repository: Arc<dyn ProjectRepository>,
    authorization_engine: Arc<dyn AuthorizationEngine>,
}

impl ProjectService {
    pub fn new(
        project_repository: Arc<dyn ProjectRepository>,
        authorization_engine: Arc<dyn AuthorizationEngine>,
    ) -> Self {
        Self {
            project_repository,
            authorization_engine,
        }
    }

    /// Read a specific project (`project_id`). `user` is used for
    /// authorization.
    pub async fn read(
        &self,
        user: &Option<User>,
        project_id: &ProjectId,
    ) -> Result<Project, ProjectServiceError> {
        let project = self.project_repository.read(project_id).await?;

        self.authorization_engine
            .can_on_instance(user, &Action::Read, &project)?;

        Ok(project)
    }

    /// List the projects that a user is allowed to view.
    pub async fn list(&self, user: &Option<User>) -> Result<Vec<Project>, ProjectServiceError> {
        self.authorization_engine
            .can_on_type(user, &Action::List, &ProjectTypeResource)?;

        let projects = self.project_repository.list().await?;

        // Filter out the projects that the user is not allowed to read.
        let allowed_projects = projects
            .into_iter()
            .filter(|p| {
                self.authorization_engine
                    .can_on_instance(user, &Action::Read, p)
                    .is_ok()
            })
            .collect();

        Ok(allowed_projects)
    }
}

#[async_trait]
impl ProjectServicePort for ProjectService {
    async fn read(
        &self,
        user: &Option<User>,
        project_id: &ProjectId,
    ) -> Result<Project, ProjectServiceError> {
        Self::read(self, user, project_id).await
    }

    async fn list(&self, user: &Option<User>) -> Result<Vec<Project>, ProjectServiceError> {
        Self::list(self, user).await
    }
}

#[cfg(test)]
mod project_asset_service_tests {
    use fake::Fake;
    use fake::Faker;

    use super::*;
    use crate::authorization::domain::error::AuthorizationEngineError;
    use crate::authorization::ports::authorization_engine::MockAuthorizationEngine;
    use crate::project::ports::project_repository::MockProjectRepository;

    mod read {

        use super::*;

        #[tokio::test]
        async fn should_return_the_correct_error_if_user_not_authenticated() {
            // Arrange
            let mut project_repository = MockProjectRepository::new();
            project_repository
                .expect_read()
                .return_const(Ok(Faker.fake()));
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can_on_instance()
                .return_const(Err(AuthorizationEngineError::NotAuthenticated));

            let project_service =
                ProjectService::new(Arc::new(project_repository), Arc::new(authorization_engine));

            // Act
            let res = project_service.read(&Faker.fake(), &Faker.fake()).await;

            // Assert
            assert!(matches!(res, Err(ProjectServiceError::NotAuthenticated)))
        }
    }

    #[tokio::test]
    async fn should_return_the_correct_error_if_user_not_authorized() {
        // Arrange
        let dummy_user = Faker.fake::<User>();

        let mut project_repository = MockProjectRepository::new();
        project_repository
            .expect_read()
            .return_const(Ok(Faker.fake()));
        let mut authorization_engine = MockAuthorizationEngine::new();
        authorization_engine
            .expect_can_on_instance()
            .return_const(Err(AuthorizationEngineError::NotAuthorized {
                user: Box::new(dummy_user.clone()),
                action: Action::Read,
                resource_identifier: Some(Faker.fake()),
                resource_type: Faker.fake(),
            }));

        let project_service =
            ProjectService::new(Arc::new(project_repository), Arc::new(authorization_engine));

        // Act
        let res = project_service.read(&Faker.fake(), &Faker.fake()).await;

        // Assert
        assert!(matches!(
            res,
            Err(ProjectServiceError::NotAuthorized { .. })
        ))
    }

    mod list {
        use std::sync::Mutex;

        use super::*;
        use crate::project::domain::Project;

        #[tokio::test]
        async fn should_return_the_projects_that_the_user_is_allowed_to_read() {
            // Arrange
            let dummy_projects = Faker.fake::<Vec<Project>>();

            let mut project_repository = MockProjectRepository::new();
            project_repository
                .expect_list()
                .return_const(Ok(dummy_projects.clone()));
            let mut authorization_engine = MockAuthorizationEngine::new();
            // The first call to the authZ engine is checking that the user is allowed to
            // list projects.
            authorization_engine
                .expect_can_on_type()
                .once()
                .return_const(Ok(()));
            // Subsequent calls determines if the user is allow to read a specific project.
            let authorization_engine_call_count = Arc::new(Mutex::new(0));
            let authorization_engine_call_count_clone =
                Arc::clone(&authorization_engine_call_count);
            authorization_engine
                .expect_can_on_instance()
                .returning(move |_, _, _| {
                    let mut count = authorization_engine_call_count_clone.lock().unwrap();
                    let res = if *count % 2 == 0 {
                        Err(AuthorizationEngineError::NotAuthorized {
                            user: Faker.fake(),
                            action: Action::List,
                            resource_identifier: Some(Faker.fake()),
                            resource_type: Faker.fake(),
                        })
                    } else {
                        Ok(())
                    };
                    *count += 1;
                    res
                });

            let project_service =
                ProjectService::new(Arc::new(project_repository), Arc::new(authorization_engine));

            // Act
            let allowed_projects = project_service.list(&Faker.fake()).await.unwrap();

            // Assert
            assert_eq!(allowed_projects.len(), dummy_projects.len() / 2);
        }

        #[tokio::test]
        async fn should_return_the_correct_error_if_user_not_allowed_to_list_projects() {
            // Arrange
            let project_repository = MockProjectRepository::new();
            let mut authorization_engine = MockAuthorizationEngine::new();
            authorization_engine
                .expect_can_on_type()
                .once()
                .return_const(Err(AuthorizationEngineError::NotAuthorized {
                    user: Faker.fake(),
                    action: Action::List,
                    resource_identifier: None,
                    resource_type: Faker.fake(),
                }));

            let project_service =
                ProjectService::new(Arc::new(project_repository), Arc::new(authorization_engine));

            // Act
            let res = project_service.list(&Faker.fake()).await;

            // Assert
            assert!(matches!(
                res,
                Err(ProjectServiceError::NotAuthorized { .. })
            ));
        }
    }
}
