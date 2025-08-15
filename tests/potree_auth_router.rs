//! Integration tests for the top-level potree-auth router.

mod test_utils;

use std::sync::LazyLock;

use axum::ServiceExt;
use axum::extract::Request;
use axum_test::TestServer;
use axum_test::transport_layer::IntoTransportLayer;
use http::StatusCode;
use http::header;
use potree_auth::potree_auth::config::PotreeAuthConfiguration;
use potree_auth::potree_auth::init_application;
use web_route::ParameterizedRoute;
use web_route::WebRoute;

use crate::test_utils::TEST_PROJECT_1_DATA_CONTENT;
use crate::test_utils::TEST_PROJECT_1_DATA_PATH;
use crate::test_utils::TEST_PROJECT_1_DATA_TYPE;
use crate::test_utils::TEST_PROJECT_1_DIR;
use crate::test_utils::TEST_PROJECT_2_DATA_PATH;
use crate::test_utils::TEST_PROJECT_2_DIR;
use crate::test_utils::TEST_PROJECT_PARENT;

static HEALTH_CHECK: LazyLock<WebRoute> = LazyLock::new(|| WebRoute::new("/_health"));
static POTREE_ASSETS: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/potree-assets/{*path}"));
static PROJECT_ASSETS: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/project-assets/{project_id}/{*path}"));
static POTREE_RENDER: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/potree/{project_id}"));
static PROJECTS_DASHBOARD: LazyLock<WebRoute> = LazyLock::new(|| WebRoute::new("/projects"));

fn test_configuration_no_idp() -> PotreeAuthConfiguration {
    PotreeAuthConfiguration {
        data_dir: TEST_PROJECT_PARENT.parse().unwrap(),
        idp: None,
    }
}

async fn initialize_application() -> impl IntoTransportLayer {
    let application = init_application(test_configuration_no_idp()).await.unwrap();

    ServiceExt::<Request>::into_make_service(application)
}

mod health_check {

    use super::*;

    #[tokio::test]
    async fn should_return_a_200() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server.get(&HEALTH_CHECK).await;

        // Assert
        response.assert_status(StatusCode::OK);
    }
}

mod potree_static_assets {
    use super::*;

    #[tokio::test]
    async fn should_return_the_asset_correctly_if_found() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server
            .get(
                &POTREE_ASSETS
                    .to_web_route(&serde_json::json!({
                        "path": "build/potree/potree.js",
                    }))
                    .unwrap(),
            )
            .await;

        // Assert
        response.assert_status(StatusCode::OK);
        assert_eq!(response.content_type(), mime::TEXT_JAVASCRIPT.as_ref());
    }

    #[tokio::test]
    async fn should_return_a_404_if_not_found() {
        // Arrange
        let non_existent_path = "build/non/existent.txt";
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server
            .get(
                &POTREE_ASSETS
                    .to_web_route(&serde_json::json!({
                        "path": non_existent_path,
                    }))
                    .unwrap(),
            )
            .await;

        // Assert
        response.assert_status(StatusCode::NOT_FOUND);
    }
}

mod project_static_assets {
    use super::*;

    #[tokio::test]
    async fn should_return_the_asset_correctly_if_found() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server
            .get(
                &PROJECT_ASSETS
                    .to_web_route(&serde_json::json!( {
                        "project_id": TEST_PROJECT_1_DIR,
                        "path": TEST_PROJECT_1_DATA_PATH,
                    }))
                    .unwrap(),
            )
            .await;

        // Assert
        response.assert_status(StatusCode::OK);
        assert_eq!(response.content_type(), TEST_PROJECT_1_DATA_TYPE.as_ref());
        assert_eq!(response.text(), TEST_PROJECT_1_DATA_CONTENT)
    }

    #[tokio::test]
    async fn should_return_an_asset_range_correctly() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server
            .get(
                &PROJECT_ASSETS
                    .to_web_route(&serde_json::json!( {
                        "project_id": TEST_PROJECT_1_DIR,
                        "path": TEST_PROJECT_1_DATA_PATH,
                    }))
                    .unwrap(),
            )
            .add_header(header::RANGE, "bytes=2-6")
            .await;

        // Assert
        response.assert_status(StatusCode::PARTIAL_CONTENT);
        response.assert_header(header::CONTENT_RANGE, "bytes 2-6/14");
        assert_eq!(response.content_type(), TEST_PROJECT_1_DATA_TYPE.as_ref());
        assert_eq!(response.text(), TEST_PROJECT_1_DATA_CONTENT[2..=6]);
    }

    #[tokio::test]
    async fn should_return_a_404_if_not_found() {
        // Arrange
        let non_existent_path = WebRoute::new("build/non/existent.txt");

        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server
            .get(
                &PROJECT_ASSETS
                    .to_web_route(&serde_json::json!( {
                        "project_id": TEST_PROJECT_1_DIR,
                        "path": non_existent_path.as_ref(),
                    }))
                    .unwrap(),
            )
            .await;

        // Assert
        response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "the route handler seems to be getting project 2 as the project_id so authZ would be fine, this test should be done with proper auth mocking"]
    async fn should_return_a_404_if_parent_directory_reference_in_path() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server
                .get(&format!(
                    "/project-assets/{TEST_PROJECT_1_DIR}/../{TEST_PROJECT_2_DIR}/{TEST_PROJECT_2_DATA_PATH}"))
                .await;

        // Assert
        response.assert_status(StatusCode::NOT_FOUND);
    }
}

mod potree_render {
    use super::*;

    #[tokio::test]
    async fn should_return_the_correct_html() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server
            .get(
                &POTREE_RENDER
                    .to_web_route(&serde_json::json!({"project_id": TEST_PROJECT_1_DIR}))
                    .unwrap(),
            )
            .await;

        // Assert
        response.assert_status(StatusCode::OK);
        assert_eq!(response.content_type(), mime::TEXT_HTML_UTF_8.to_string());
        assert!(response.text().contains(TEST_PROJECT_1_DIR))
    }

    #[tokio::test]
    async fn should_redirect_to_404_if_not_exist() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server
            .get(
                &POTREE_RENDER
                    .to_web_route(&serde_json::json!({"project_id": "not-exist"}))
                    .unwrap(),
            )
            .await;

        // Assert
        response.assert_status(StatusCode::SEE_OTHER);
    }
}

mod projects_dashboard {
    use super::*;

    #[tokio::test]
    async fn should_return_the_correct_html() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server.get(&PROJECTS_DASHBOARD).await;

        // Assert
        response.assert_status(StatusCode::OK);
        assert_eq!(response.content_type(), mime::TEXT_HTML_UTF_8.to_string());

        // Check that the projects are listed
        assert!(response.text().contains("Project 1"));
        assert!(response.text().contains("Project 2"));
    }
}

mod secure_headers {
    use super::*;

    #[tokio::test]
    async fn should_have_secure_headers_middleware_active() {
        // Arrange
        let test_server = TestServer::new(initialize_application().await).unwrap();

        // Act
        let response = test_server.get(&HEALTH_CHECK).await;

        // Assert
        //
        // Note: This is not an exhaustive list, just checking that the security headers
        // middleware is active.
        response.assert_contains_header(http::header::CONTENT_SECURITY_POLICY);
        response.assert_contains_header(http::header::STRICT_TRANSPORT_SECURITY);
        response.assert_contains_header("permissions-policy");
    }
}
