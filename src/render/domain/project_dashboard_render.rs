use askama::Template;
use web_route::ParameterizedRoute;
use web_route::WebRoute;

use crate::common::domain::value_objects::ProjectDescription;
use crate::common::domain::value_objects::ProjectName;
use crate::render::domain::error::RenderDomainError;

/// Represents the the Project Dashboard page.
///
/// Displays all of the projects for a user.
#[derive(Debug, Template)]
#[template(path = "project_dashboard/index.html")]
pub struct ProjectDashboard {
    /// The projects that the user is allowed to read.
    pub projects: Vec<Project>,
}

impl ProjectDashboard {
    /// Creates a new [`ProjectDashboard`] by converting from domain
    /// [`Project`][`crate::common::domain::project::Project`]s. Calculates the
    /// route to which the user should be redirected to view a project.
    pub fn from_domain_projects<P>(
        projects: P,
        default_render_route: &ParameterizedRoute,
    ) -> Result<Self, RenderDomainError>
    where
        P: IntoIterator<Item = crate::common::domain::project::Project>,
    {
        let projects = projects
            .into_iter()
            .map(|p| Project::from_domain_project(p, default_render_route))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { projects })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    /// A human readable name for the project.
    pub name: ProjectName,

    /// Optional additional context about the project.
    pub description: Option<ProjectDescription>,

    /// The route to which the user should be redirected to render the project.
    pub render_route: WebRoute,
}

impl Project {
    /// Creates a new [`Project`] by converting from a domain
    /// [`Project`][`crate::common::domain::project::Project`].
    ///
    /// If not populated calculates the default route to which the user should
    /// be redirected to view a project.
    ///
    /// `default_render_route` is assumed to have a `{project_id}` token.
    pub fn from_domain_project(
        project: crate::common::domain::project::Project,
        default_render_route: &ParameterizedRoute,
    ) -> Result<Self, RenderDomainError> {
        let render_route = default_render_route
            .to_web_route(&serde_json::json!({"project_id": project.id}))
            .map_err(|_e| RenderDomainError::InvalidRoutePopulation {
                route: default_render_route.clone(),
            })?;

        Ok(Self {
            name: project.name,
            description: project.description,
            render_route,
        })
    }
}

#[cfg(test)]
mod project_tests {
    use fake::Fake;
    use fake::Faker;

    use super::*;

    mod from_domain_project {

        use super::*;

        #[test]
        fn should_create_a_valid_render_route() {
            // Arrange
            let domain_project = Faker.fake::<crate::common::domain::project::Project>();
            let default_render_route =
                ParameterizedRoute::new(Faker.fake::<WebRoute>()).join("/{project_id}");

            // Act
            let project =
                Project::from_domain_project(domain_project.clone(), &default_render_route)
                    .unwrap();

            // Assert
            assert!(
                project
                    .render_route
                    .as_ref()
                    .contains(domain_project.id.as_str())
            );
        }
    }
}
