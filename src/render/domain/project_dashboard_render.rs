use askama::Template;

use crate::common::domain::project::Project;

/// Represents the the Project Dashboard page.
///
/// Displays all of the projects for a user.
#[derive(Debug, Template)]
#[template(path = "project_dashboard.html")]
pub struct ProjectDashboard {
    /// The projects that the user is allowed to read.
    pub projects: Vec<Project>,
}
