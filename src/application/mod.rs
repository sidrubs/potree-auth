use std::sync::Arc;

use axum::Router;
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use routes::{build_router, callback_route};

use crate::{
    config::ApplicationConfiguration,
    error::ApplicationError,
    services::{
        authentication::{
            AuthenticationService, oidc_authentication::OidcAuthenticationService,
            unimplemented_authentication::UnimplementedAuthenticationService,
        },
        authorization::{
            AuthorizationService, always_allowed::AlwaysAllowedAuthorizationService,
            basic_authorization::SimpleAuthorizationService,
        },
        potree_assets::embedded::EmbeddedPotreeAssetService,
        project::manifest_file::ManifestFileProjectService,
        project_assets::serve_dir::ServeDirProjectAssets,
    },
};

mod extractors;
mod middleware;
mod routes;
mod utils;
mod views;

/// Sets up the required services and builds the application routes configured
/// as per the `config`.
pub async fn initialize_application(
    config: &ApplicationConfiguration,
) -> Result<Router, ApplicationError> {
    // Set up services.
    let project_service = Arc::new(ManifestFileProjectService::new(&config.projects_dir));
    let project_asset_service = Arc::new(ServeDirProjectAssets::new(&config.projects_dir));
    let potree_asset_service = Arc::new(EmbeddedPotreeAssetService);

    // Determine which authentication and authorization service to initialize based
    // on the configuration.
    let (authentication_service, authorization_service): (
        Arc<dyn AuthenticationService>,
        Arc<dyn AuthorizationService>,
    ) = if let Some(idp_configuration) = config.idp.clone() {
        // Use IdP authenticated routes.
        let authentication_service = Arc::new(
            OidcAuthenticationService::new(
                IssuerUrl::from_url(idp_configuration.idp_url),
                RedirectUrl::from_url(
                    idp_configuration
                        .external_url
                        .join(&callback_route())
                        .map_err(|err| {
                            ApplicationError::Initialization(format!(
                                "unable to generate the OIDC callback URL: {err}"
                            ))
                        })?,
                ),
                ClientId::new(idp_configuration.client_id),
                ClientSecret::new(idp_configuration.client_secret),
                idp_configuration.groups_claim,
            )
            .await?,
        );

        // Require authorization.
        let authorization_service = Arc::new(SimpleAuthorizationService);

        (authentication_service, authorization_service)
    } else {
        // Don't use authentication or authorization.
        (
            Arc::new(UnimplementedAuthenticationService),
            Arc::new(AlwaysAllowedAuthorizationService),
        )
    };

    Ok(build_router(
        authorization_service,
        authentication_service,
        project_service,
        project_asset_service,
        potree_asset_service,
    ))
}

/// End-to-end tests for the application stack.
#[cfg(test)]
mod router_integration_tests {
    use axum_test::TestServer;
    use http::{StatusCode, header};

    use crate::test_utils::{
        TEST_PROJECT_1_DATA_CONTENT, TEST_PROJECT_1_DATA_PATH, TEST_PROJECT_1_DATA_TYPE,
        TEST_PROJECT_1_DIR, TEST_PROJECT_2_DATA_PATH, TEST_PROJECT_2_DIR, TEST_PROJECT_PARENT,
    };

    use super::*;

    const TEST_HEALTH_CHECK: &str = "/_health";
    const TEST_POTREE_STATIC_ASSETS: &str = "/static/potree";
    const TEST_PROJECT: &str = "/project";

    mod health_check {

        use super::*;

        #[tokio::test]
        async fn should_return_a_200() {
            // Arrange
            let test_server = TestServer::new(
                initialize_application(&ApplicationConfiguration::dummy_with_no_idp())
                    .await
                    .unwrap(),
            )
            .unwrap();

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
            let test_server = TestServer::new(
                initialize_application(&ApplicationConfiguration::dummy_with_no_idp())
                    .await
                    .unwrap(),
            )
            .unwrap();
            // Act
            let response = test_server
                .get(&format!(
                    "{TEST_POTREE_STATIC_ASSETS}/build/potree/potree.js"
                ))
                .await;

            // Assert
            response.assert_status(StatusCode::OK);
            assert_eq!(response.content_type(), mime::TEXT_JAVASCRIPT.as_ref());
        }

        #[tokio::test]
        async fn should_return_a_404_if_not_found() {
            // Arrange
            let non_existent_path = "build/non/existent.txt";
            let test_server = TestServer::new(
                initialize_application(&ApplicationConfiguration::dummy_with_no_idp())
                    .await
                    .unwrap(),
            )
            .unwrap();
            // Act
            let response = test_server
                .get(&format!("{TEST_POTREE_STATIC_ASSETS}/{non_existent_path}"))
                .await;

            // Assert
            response.assert_status(StatusCode::NOT_FOUND);
            assert_eq!(
                response.text(),
                format!("unable to find static asset: {non_existent_path}")
            );
        }
    }

    mod project_static_assets {
        use super::*;

        #[tokio::test]
        async fn should_return_the_asset_correctly_if_found() {
            // Arrange
            let config = ApplicationConfiguration {
                projects_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(&format!(
                    "{TEST_PROJECT}/{TEST_PROJECT_1_DIR}/assets/{TEST_PROJECT_1_DATA_PATH}"
                ))
                .await;

            // Assert
            response.assert_status(StatusCode::OK);
            assert_eq!(response.content_type(), TEST_PROJECT_1_DATA_TYPE.as_ref());
            assert_eq!(response.text(), TEST_PROJECT_1_DATA_CONTENT)
        }

        #[tokio::test]
        async fn should_return_an_asset_range_correctly() {
            // Arrange
            let config = ApplicationConfiguration {
                projects_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(&format!(
                    "{TEST_PROJECT}/{TEST_PROJECT_1_DIR}/assets/{TEST_PROJECT_1_DATA_PATH}"
                ))
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
            let non_existent_path = "build/non/existent.txt";

            let config = ApplicationConfiguration {
                projects_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(&format!(
                    "{TEST_PROJECT}/{TEST_PROJECT_1_DIR}/assets/{non_existent_path}"
                ))
                .await;

            // Assert
            response.assert_status(StatusCode::NOT_FOUND);
            assert_eq!(
                response.text(),
                format!("unable to find project asset: {TEST_PROJECT_1_DIR}/{non_existent_path}")
            );
        }

        #[tokio::test]
        async fn should_return_a_404_if_parent_directory_reference_in_path() {
            // Arrange
            let config = ApplicationConfiguration {
                projects_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(&format!(
                    "{TEST_PROJECT}/{TEST_PROJECT_1_DIR}/assets/../{TEST_PROJECT_2_DIR}/{TEST_PROJECT_2_DATA_PATH}"
                ))
                .await;

            // Assert
            response.assert_status(StatusCode::NOT_FOUND);
        }

        #[tokio::test]
        async fn should_contain_cache_control_response_header() {
            // Arrange
            let config = ApplicationConfiguration {
                projects_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(&format!(
                    "{TEST_PROJECT}/{TEST_PROJECT_1_DIR}/assets/{TEST_PROJECT_1_DATA_PATH}"
                ))
                .await;

            // Assert
            assert!(response.headers().contains_key(header::CACHE_CONTROL));
        }
    }

    mod potree_render {
        use super::*;

        #[tokio::test]
        async fn should_return_the_correct_html() {
            // Arrange
            let config = ApplicationConfiguration {
                projects_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(&format!("{TEST_PROJECT}/{TEST_PROJECT_1_DIR}"))
                .await;

            // Assert
            response.assert_status(StatusCode::OK);
            assert_eq!(response.content_type(), mime::TEXT_HTML_UTF_8.to_string());
            assert!(response.text().contains(TEST_PROJECT_1_DIR))
        }
    }
}
