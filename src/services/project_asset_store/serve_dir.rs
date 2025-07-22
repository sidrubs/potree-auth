use std::path::{Component, Path, PathBuf};

use async_trait::async_trait;
use http::{HeaderMap, Request, Response, StatusCode};
use http_body_util::BodyExt;
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

use crate::{
    domain::{ResourceType, StaticAsset},
    error::ApplicationError,
};

use super::ProjectAssetStore;

/// An implementation of the [`ProjectAssetService`]. It uses
/// [`tower_http::services::ServeFile`] under the hood, as its logic is robust
/// and well tested.
#[derive(Debug, Clone)]
pub struct ServeDirProjectAssets {
    /// The root directory that all the asset paths are relative to.
    base_dir: PathBuf,
}

impl ServeDirProjectAssets {
    /// Create a new [`ServeDirProjectAssets`] struct. The `base_dir` being the
    /// parent directory to all of the project directories.
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    #[tracing::instrument]
    pub async fn get_asset(
        &self,
        path: &str,
        request_headers: Option<HeaderMap>,
    ) -> Result<StaticAsset, ApplicationError> {
        // Ensure the `path` does not contain parent dir references as this could lead
        // to illegal path traversal.
        if contains_parent_dir_reference(path) {
            return Err(ApplicationError::DisallowedAction(
                "the `path` may not contain parent directory references".to_owned(),
            ));
        }

        // Build empty request containing only headers, as the headers are used by
        // [`ServeFile`] to determine how the data should be formatted.
        let mut request = Request::new(());
        if let Some(request_headers) = request_headers {
            *request.headers_mut() = request_headers;
        }

        let file_path = self.base_dir.join(path);

        // Use `ServeFile` to fetch the file based on the request headers. I feel that
        // this is a bit clunky an inefficient, but it gives me a nice consistent
        // abstraction in the project so I like it.
        let serve_file = ServeFile::new(file_path.to_string_lossy().as_ref());
        let response = serve_file.oneshot(request).await.map_err(|_err| {
            ApplicationError::ResourceNotFound {
                resource_name: path.to_owned(),
                resource_type: ResourceType::ProjectAsset,
            }
        })?;

        // Because this is a response which is always successful, we need to check the
        // response status code.
        if response.status() != StatusCode::OK && response.status() != StatusCode::PARTIAL_CONTENT {
            return Err(ApplicationError::ResourceNotFound {
                resource_name: path.to_owned(),
                resource_type: ResourceType::ProjectAsset,
            });
        }

        // Deconstruct the original response
        let (parts, body) = response.into_parts();

        // Collect the body into Bytes
        let bytes = body
            .collect()
            .await
            .map_err(|_err| {
                ApplicationError::ServerError(format!("unable to convert `{path}` to bytes"))
            })?
            .to_bytes();

        // Reconstruct the response with the new Bytes body
        let response = Response::from_parts(parts, bytes);

        Ok(StaticAsset(response))
    }
}

#[async_trait]
impl ProjectAssetStore for ServeDirProjectAssets {
    async fn get_asset(
        &self,
        path: &str,
        request_headers: Option<HeaderMap>,
    ) -> Result<StaticAsset, ApplicationError> {
        Self::get_asset(self, path, request_headers).await
    }
}

/// Determines if a `path` contains any parent directory references.
fn contains_parent_dir_reference<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .components()
        .any(|component| component == Component::ParentDir)
}

#[cfg(test)]
mod serve_dir_project_assets_tests {
    use fake::{Fake, Faker};

    use super::*;

    const TEST_FILE_CONTENT: &str = "this is a test string";

    /// Writes an asset to a text file in the `assets_dir`. Returns the path to
    /// the created asset within the `assets_dir`.
    fn create_asset_file<P: AsRef<Path>>(assets_dir: P) -> PathBuf {
        let asset_path = PathBuf::new().join(Faker.fake::<String>());
        let asset_name = "test.txt";
        let absolute_path = PathBuf::new().join(&assets_dir).join(&asset_path);

        std::fs::create_dir(&absolute_path).unwrap();

        std::fs::write(absolute_path.join(asset_name), TEST_FILE_CONTENT).unwrap();

        asset_path.join(asset_name)
    }

    mod get_asset {
        use http::{HeaderValue, header};

        use super::*;

        #[tokio::test]
        async fn should_return_a_valid_asset_if_it_exists() {
            // Arrange
            let assets_dir = tempfile::tempdir().unwrap();
            let asset_service = ServeDirProjectAssets::new(&assets_dir);

            let asset_path = create_asset_file(&assets_dir);

            // Act
            let static_asset = asset_service
                .get_asset(
                    asset_path.to_string_lossy().as_ref(),
                    Some(HeaderMap::new()),
                )
                .await
                .expect("unable to find asset");

            // Assert
            assert_eq!(
                static_asset.0.headers().get(header::CONTENT_TYPE).unwrap(),
                mime::TEXT_PLAIN.as_ref()
            );
            assert_eq!(
                String::from_utf8_lossy(&static_asset.data()),
                TEST_FILE_CONTENT.to_owned()
            )
        }

        #[tokio::test]
        async fn should_return_a_valid_partial_asset_if_it_exists() {
            // Arrange
            let assets_dir = tempfile::tempdir().unwrap();
            let asset_service = ServeDirProjectAssets::new(&assets_dir);

            let asset_path = create_asset_file(&assets_dir);

            let mut headers = HeaderMap::new();
            headers.append(header::RANGE, HeaderValue::from_static("bytes=3-7"));

            // Act
            let static_asset = asset_service
                .get_asset(asset_path.to_string_lossy().as_ref(), Some(headers))
                .await
                .expect("unable to find asset");

            // Assert
            assert_eq!(
                static_asset.0.headers().get(header::CONTENT_TYPE).unwrap(),
                mime::TEXT_PLAIN.as_ref()
            );
            assert_eq!(
                String::from_utf8_lossy(&static_asset.data()),
                TEST_FILE_CONTENT[3..=7].to_owned()
            )
        }

        #[tokio::test]
        async fn should_return_correct_error_if_asset_does_not_exist() {
            // Arrange
            let assets_dir = tempfile::tempdir().unwrap();
            let asset_service = ServeDirProjectAssets::new(&assets_dir);

            let non_existent_path = "build/non/existent.txt";

            // Act
            let res = asset_service
                .get_asset(non_existent_path, Some(HeaderMap::new()))
                .await;

            // Assert
            assert!(
                matches!(res, Err(ApplicationError::ResourceNotFound { resource_name, resource_type }) if resource_name == non_existent_path && resource_type == ResourceType::ProjectAsset)
            )
        }

        #[test_case::test_case("../build/non/existent.txt" ; "at beginning of path")]
        #[test_case::test_case("build/../non/existent.txt" ; "in middle of path")]
        #[tokio::test]
        async fn should_return_correct_error_if_path_contains_parent_dir_ref(invalid_path: &str) {
            // Arrange
            let assets_dir = tempfile::tempdir().unwrap();
            let asset_service = ServeDirProjectAssets::new(&assets_dir);

            // Act
            let res = asset_service
                .get_asset(invalid_path, Some(HeaderMap::new()))
                .await;

            // Assert
            assert!(matches!(res, Err(ApplicationError::DisallowedAction(_))))
        }
    }
}
