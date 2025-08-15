use axum::response::Redirect;

pub fn redirect_to_login(login_route: &str, page_path: &str) -> Redirect {
    Redirect::to(&format!("{}?next_path={}", login_route, page_path))
}
