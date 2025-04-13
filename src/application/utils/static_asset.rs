use axum::response::{IntoResponse, Response};
use http::header;

use crate::domain::StaticAsset;

/// Enables a [`StaticAsset`] to be easily converted into an `axum` HTTP
/// response.
impl IntoResponse for StaticAsset {
    fn into_response(self) -> Response {
        ([(header::CONTENT_TYPE, self.mime.as_ref())], self.data).into_response()
    }
}
