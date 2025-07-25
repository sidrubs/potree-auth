use http::StatusCode;

/// Responds with an empty 200. Used to check if the server is ready to accept
/// requests.
pub(crate) async fn health_check() -> StatusCode {
    StatusCode::OK
}
