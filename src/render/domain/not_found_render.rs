use askama::Template;

/// Displays a 404 page.
#[derive(Debug, Template)]
#[template(path = "error/404.html")]
pub struct NotFound;
