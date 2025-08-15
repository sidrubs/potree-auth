use askama::Template;

/// Displays a 404 page.
#[derive(Debug, Template)]
#[template(path = "404.html")]
pub struct NotFound;
