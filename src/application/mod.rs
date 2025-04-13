use axum::Router;
use routes::build_router;

use crate::{
    config::ApplicationConfiguration,
    services::{
        authorization::basic_authorization::SimpleAuthorizationService,
        potree_assets::embedded::EmbeddedPotreeAssetService,
        project::manifest_file::ManifestFileProjectService,
    },
};

mod extractors;
mod routes;
mod utils;

/// Sets up the required services and builds the application routes configured
/// as per the `config`.
pub fn initialize_application(config: &ApplicationConfiguration) -> Router {
    // Set up services.
    let authorization_service = SimpleAuthorizationService;
    let project_service = ManifestFileProjectService::new(&config.projects_dir);
    let potree_asset_service = EmbeddedPotreeAssetService;

    build_router(authorization_service, project_service, potree_asset_service)
}

/// End-to-end tests for the application stack.
#[cfg(test)]
mod router_integration_tests {
    use axum_test::TestServer;
    use fake::{Fake, Faker};
    use http::StatusCode;

    use super::*;

    const TEST_HEALTH_CHECK: &str = "/_health";
    const POTREE_STATIC_ASSETS: &str = "/static/potree";

    mod health_check {

        use super::*;

        #[tokio::test]
        async fn should_return_a_200() {
            // Arrange
            let test_server = TestServer::new(initialize_application(&Faker.fake())).unwrap();

            // Act
            let response = test_server.get(TEST_HEALTH_CHECK).await;

            // Assert
            response.assert_status(StatusCode::OK);
        }
    }

    mod potree_static_assets {

        use super::*;

        #[tokio::test]
        async fn should_return_the_asset_correctly_if_found() {
            // Arrange
            let test_server = TestServer::new(initialize_application(&Faker.fake())).unwrap();

            // Act
            let response = test_server
                .get(&format!("{POTREE_STATIC_ASSETS}/build/potree/potree.js"))
                .await;

            // Assert
            response.assert_status(StatusCode::OK);
            assert_eq!(response.content_type(), mime::TEXT_JAVASCRIPT.as_ref());
        }

        #[tokio::test]
        async fn should_return_a_404_if_not_found() {
            // Arrange
            let non_existent_path = "build/non/existent.txt";
            let test_server = TestServer::new(initialize_application(&Faker.fake())).unwrap();

            // Act
            let response = test_server
                .get(&format!("{POTREE_STATIC_ASSETS}/{non_existent_path}"))
                .await;

            // Assert
            response.assert_status(StatusCode::NOT_FOUND);
            assert_eq!(
                response.text(),
                format!("unable to find static asset: {non_existent_path}")
            );
        }
    }
}
