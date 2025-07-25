use askama::Template;
use web_route::WebRoute;

use crate::domain::value_objects::ProjectName;

/// Represents the the `potree` render page. Populates and renders the
/// `potree_render.html` template.
///
/// This assumes that there is a `potree` config file fully defining the
/// project.
#[derive(Debug, Template)]
#[template(path = "potree_render.html")]
pub struct PotreeRender {
    /// The title of the project to render.
    pub project_title: ProjectName,

    /// The path where the `potree` static assets are served.
    pub potree_static_assets_path: String,

    /// The path to the `potree` project config file.
    pub potree_config_path: WebRoute,
}
