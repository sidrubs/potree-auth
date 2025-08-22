use std::collections::HashMap;

use http::HeaderValue;
use tower_helmet::IntoHeader;
use tower_helmet::header::ContentSecurityPolicy;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::common::utils::http::render_error::RenderError;

/// Adds a more lenient CSP for potree rendering pages as it has inline JS.
pub fn set_potree_csp() -> Result<SetResponseHeaderLayer<HeaderValue>, RenderError> {
    let mut directives = HashMap::new();
    directives.insert("script-src", vec!["'self'", "'unsafe-inline'"]);
    let csp = ContentSecurityPolicy {
        directives,
        ..Default::default()
    };

    Ok(SetResponseHeaderLayer::overriding(
        http::header::CONTENT_SECURITY_POLICY,
        csp.header_value()
            .map_err(|_e| RenderError::ServerConfiguration {
                message: "invalid CSP header value".to_owned(),
            })?,
    ))
}
