use std::collections::HashMap;

use http::HeaderValue;
use tower_helmet::IntoHeader;
use tower_helmet::header::ContentSecurityPolicy;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::common::utils::http::initialization_error::InitializationError;

/// Adds a more lenient CSP for potree rendering pages as it has inline JS.
pub fn set_potree_csp() -> Result<SetResponseHeaderLayer<HeaderValue>, InitializationError> {
    let mut directives = HashMap::new();
    directives.insert("script-src", vec!["'self'", "'unsafe-inline'"]);
    let csp = ContentSecurityPolicy {
        directives,
        ..Default::default()
    };

    Ok(SetResponseHeaderLayer::overriding(
        http::header::CONTENT_SECURITY_POLICY,
        csp.header_value()
            .map_err(|_e| InitializationError::Middleware {
                middleware_name: "potree csp header".to_owned(),
                message: "invalid CSP header value".to_owned(),
            })?,
    ))
}
