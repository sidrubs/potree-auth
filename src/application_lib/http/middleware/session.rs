use axum::Router;
use time::Duration;
use tower_sessions::Expiry;
use tower_sessions::MemoryStore;
use tower_sessions::SessionManagerLayer;
use tower_sessions::cookie;

/// Applies a web session management layer to the router.
pub fn apply_session_layer(router: Router, session_expiry: Duration) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_http_only(true)
        // Else the redirect from the IdP does not work?
        .with_same_site(cookie::SameSite::None)
        .with_expiry(Expiry::OnInactivity(session_expiry));

    router.layer(session_layer)
}
