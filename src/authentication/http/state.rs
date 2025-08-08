use super::super::application::service::AuthenticationService;

#[derive(Debug, Clone)]
pub struct State {
    pub authentication_service: AuthenticationService,
}
