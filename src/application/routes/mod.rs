mod health_check;
mod state;

use std::sync::Arc;

use axum::{Extension, Router, routing::get};
use state::ApplicationState;

use crate::services::{
    authorization::AuthorizationService, potree_assets::PotreeAssetService, project::ProjectService,
};

const HEALTH_CHECK: &str = "/_health";

/// Initializes the application router, its state, and all of its routes.
pub fn build_router<AZ, P, PA>(
    authorization_service: AZ,
    project_service: P,
    potree_asset_service: PA,
) -> Router
where
    AZ: AuthorizationService,
    P: ProjectService,
    PA: PotreeAssetService,
{
    // Initialize application state.
    let state = ApplicationState {
        authorization_service: Arc::new(authorization_service),
        project_service: Arc::new(project_service),
        potree_asset_service: Arc::new(potree_asset_service),
    };

    // Build the router.
    let router = Router::new()
        .route(HEALTH_CHECK, get(health_check::health_check))
        .layer(Extension(state));

    router
}

/// Integration tests for the entire router stack.
#[cfg(test)]
mod router_integration_tests {
    use axum_test::TestServer;
    use http::StatusCode;

    use super::*;

    const TEST_HEALTH_CHECK: &str = "/_health";

    mod health_check {

        use crate::services::{
            authorization::MockAuthorizationService, potree_assets::MockPotreeAssetService,
            project::MockProjectService,
        };

        use super::*;

        #[tokio::test]
        async fn should_return_a_200() {
            // Arrange
            let authorization_service = MockAuthorizationService::new();
            let project_service = MockProjectService::new();
            let potree_asset_service = MockPotreeAssetService::new();
            let test_server = TestServer::new(build_router(
                authorization_service,
                project_service,
                potree_asset_service,
            ))
            .unwrap();

            // Act
            let response = test_server.get(TEST_HEALTH_CHECK).await;

            // Assert
            response.assert_status(StatusCode::OK);
        }
    }
}
