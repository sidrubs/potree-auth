use axum::Router;
use tower_helmet::HelmetLayer;
use tower_http::set_header::SetResponseHeaderLayer;

/// Adds security headers middleware to the `router`.
///
/// Adds OWASP's recommended [secure headers] to HTTP responses.
///
/// [secure headers]: https://owasp.org/www-project-secure-headers/
pub fn apply_secure_headers_middleware(router: Router) -> Router {
    let helmet_layer = HelmetLayer::with_defaults();

    let permissions_policy_layer = SetResponseHeaderLayer::if_not_present(
        http::HeaderName::from_static("permissions-policy"),
        http::HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );

    router.layer(helmet_layer).layer(permissions_policy_layer)
}
