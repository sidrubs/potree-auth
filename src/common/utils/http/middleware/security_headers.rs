use axum::Router;
use http::HeaderValue;
use tower_helmet::HelmetLayer;
use tower_helmet::IntoHeader;
use tower_helmet::header::ContentSecurityPolicy;
use tower_http::set_header::SetResponseHeaderLayer;

use super::super::initialization_error::InitializationError;

/// Adds security headers middleware to the `router`.
///
/// Adds OWASP's recommended [secure headers] to HTTP responses.
///
/// [secure headers]: https://owasp.org/www-project-secure-headers/
pub fn apply_secure_headers_middleware(router: Router) -> Result<Router, InitializationError> {
    let helmet_layer = helmet_layer();

    let csp_layer = csp_layer()?;

    let permissions_policy_layer = SetResponseHeaderLayer::if_not_present(
        http::HeaderName::from_static("permissions-policy"),
        http::HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );

    Ok(router
        .layer(helmet_layer)
        .layer(csp_layer)
        .layer(permissions_policy_layer))
}

/// Initialize the helmet layer without CSP.
///
/// Otherwise it overrides the CSP set for individual routes.
fn helmet_layer() -> HelmetLayer {
    let mut helmet_layer = HelmetLayer::with_defaults();
    helmet_layer.remove(http::header::CONTENT_SECURITY_POLICY);

    helmet_layer
}

/// Initialize a default CSP layer that is added if one is not already present.
fn csp_layer() -> Result<SetResponseHeaderLayer<HeaderValue>, InitializationError> {
    let csp = ContentSecurityPolicy::default();

    Ok(SetResponseHeaderLayer::if_not_present(
        http::header::CONTENT_SECURITY_POLICY,
        csp.header_value()
            .map_err(|_e| InitializationError::Middleware {
                middleware_name: "security headers".to_owned(),
                message: "invalid CSP header value".to_owned(),
            })?,
    ))
}
