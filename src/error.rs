/// Describes all the errors that can be expected by the application.
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("{user_name} is not authorized to view the {project_name} project")]
    NotAuthorized {
        user_name: String,
        project_name: String,
    },
}
