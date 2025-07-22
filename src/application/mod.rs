use std::sync::Arc;

use axum::{Router, ServiceExt, extract::Request, routing::IntoMakeService};
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use routes::build_router;
use tower_http::normalize_path::NormalizePath;

use crate::{
    application::routes::AUTH_CALLBACK,
    config::ApplicationConfiguration,
    error::ApplicationError,
    services::{
        authentication::{
            AuthenticationService, no_op::NoOpAuthenticationService,
            oidc::OidcAuthenticationService,
        },
        authorization::{
            AuthorizationService, basic_authorization::SimpleAuthorizationService,
            no_op::NoOpAuthorizationService,
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
) -> Result<IntoMakeService<NormalizePath<Router>>, ApplicationError> {
    // Set up services.
    let project_service = Arc::new(ManifestFileProjectService::new(&config.data_dir));
    let project_asset_service = Arc::new(ServeDirProjectAssets::new(&config.data_dir));
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
                        .join(&AUTH_CALLBACK)
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
            Arc::new(NoOpAuthenticationService),
            Arc::new(NoOpAuthorizationService),
        )
    };

    let router = build_router(
        authorization_service,
        authentication_service,
        project_service,
        project_asset_service,
        potree_asset_service,
    );

    let service = ServiceExt::<Request>::into_make_service(router);

    Ok(service)
}

/// End-to-end tests for the application stack.
#[cfg(test)]
mod router_integration_tests {
    use axum_test::TestServer;
    use http::{StatusCode, header};

    use super::*;
    use crate::application::routes::{
        HEALTH_CHECK, POTREE_ASSETS, POTREE_UI_PROJECT, PROJECT_ASSETS,
    };

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
            let test_server = TestServer::new(
                initialize_application(&ApplicationConfiguration::dummy_with_no_idp())
                    .await
                    .unwrap(),
            )
            .unwrap();

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
            let test_server = TestServer::new(
                initialize_application(&ApplicationConfiguration::dummy_with_no_idp())
                    .await
                    .unwrap(),
            )
            .unwrap();

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
            assert_eq!(
                response.text(),
                format!("unable to find static asset: {non_existent_path}")
            );
        }
    }

    mod project_static_assets {
        use std::ops::Deref;

        use web_route::WebRoute;

        use crate::{
            application::routes::ProjectAssetParams,
            domain::value_objects::ProjectId,
            test_utils::{
                TEST_PROJECT_1_DATA_CONTENT, TEST_PROJECT_1_DATA_PATH, TEST_PROJECT_1_DATA_TYPE,
                TEST_PROJECT_1_DIR, TEST_PROJECT_2_DATA_PATH, TEST_PROJECT_2_DIR,
                TEST_PROJECT_PARENT,
            },
        };

        use super::*;

        #[tokio::test]
        async fn should_return_the_asset_correctly_if_found() {
            // Arrange
            let config = ApplicationConfiguration {
                data_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(
                    &PROJECT_ASSETS
                        .to_web_route(&ProjectAssetParams {
                            project_id: ProjectId::new(TEST_PROJECT_1_DIR.to_owned()),
                            path: WebRoute::new(TEST_PROJECT_1_DATA_PATH),
                        })
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
            let config = ApplicationConfiguration {
                data_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(
                    &PROJECT_ASSETS
                        .to_web_route(&ProjectAssetParams {
                            project_id: ProjectId::new(TEST_PROJECT_1_DIR.to_owned()),
                            path: WebRoute::new(TEST_PROJECT_1_DATA_PATH),
                        })
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

            let config = ApplicationConfiguration {
                data_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(
                    &PROJECT_ASSETS
                        .to_web_route(&ProjectAssetParams {
                            project_id: ProjectId::new(TEST_PROJECT_1_DIR.to_owned()),
                            path: non_existent_path.clone(),
                        })
                        .unwrap(),
                )
                .await;

            // Assert
            response.assert_status(StatusCode::NOT_FOUND);
            assert_eq!(
                response.text(),
                format!(
                    "unable to find project asset: {}{}",
                    TEST_PROJECT_1_DIR,
                    non_existent_path.deref()
                )
            );
        }

        #[tokio::test]
        #[ignore = "the route handler seems to be getting project 2 as the project_id so authZ would be fine, this test should be done with proper auth mocking"]
        async fn should_return_a_404_if_parent_directory_reference_in_path() {
            // Arrange
            let config = ApplicationConfiguration {
                data_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(&format!(
                    "/project-assets/{TEST_PROJECT_1_DIR}/../{TEST_PROJECT_2_DIR}/{TEST_PROJECT_2_DATA_PATH}"))
                .await;

            // Assert
            response.assert_status(StatusCode::NOT_FOUND);
        }

        #[tokio::test]
        async fn should_contain_cache_control_response_header() {
            // Arrange
            let config = ApplicationConfiguration {
                data_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(
                    &PROJECT_ASSETS
                        .to_web_route(&ProjectAssetParams {
                            project_id: ProjectId::new(TEST_PROJECT_1_DIR.to_owned()),
                            path: WebRoute::new(TEST_PROJECT_1_DATA_PATH),
                        })
                        .unwrap(),
                )
                .await;

            // Assert
            assert!(response.headers().contains_key(header::CACHE_CONTROL));
        }
    }

    mod potree_render {
        use crate::test_utils::{TEST_PROJECT_1_DIR, TEST_PROJECT_PARENT};

        use super::*;

        #[tokio::test]
        async fn should_return_the_correct_html() {
            // Arrange
            let config = ApplicationConfiguration {
                data_dir: TEST_PROJECT_PARENT.parse().unwrap(),
                ..ApplicationConfiguration::dummy_with_no_idp()
            };
            let test_server =
                TestServer::new(initialize_application(&config).await.unwrap()).unwrap();

            // Act
            let response = test_server
                .get(
                    &POTREE_UI_PROJECT
                        .to_web_route(&serde_json::json!({"project_id": TEST_PROJECT_1_DIR}))
                        .unwrap(),
                )
                .await;

            // Assert
            response.assert_status(StatusCode::OK);
            assert_eq!(response.content_type(), mime::TEXT_HTML_UTF_8.to_string());
            assert!(response.text().contains(TEST_PROJECT_1_DIR))
        }
    }
}
