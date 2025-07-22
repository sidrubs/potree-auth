use axum::response::IntoResponse;
use axum::response::Response;

use crate::domain::StaticAsset;

/// Enables a [`StaticAsset`] to be easily converted into an `axum` HTTP
/// response.
impl IntoResponse for StaticAsset {
    fn into_response(self) -> Response {
        let response = self.0;
        let (parts, body) = response.into_parts();
        Response::from_parts(parts, axum::body::Body::from(body))
    }
}
