use http::HeaderValue;
use tower_http::set_header::SetResponseHeaderLayer;

/// Sets a cache-control response header indicating that responses should not be
/// cached anywhere.
pub fn set_cache_control() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        http::header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    )
}
