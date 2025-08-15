use axum::response::Redirect;

use crate::render::http::router::NOT_FOUND;

pub fn redirect_to_login(login_route: &str, page_path: &str) -> Redirect {
    Redirect::to(&format!("{}?next_path={}", login_route, page_path))
}

pub fn redirect_to_404() -> Redirect {
    Redirect::to(&NOT_FOUND)
}
