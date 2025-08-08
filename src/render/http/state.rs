use web_route::WebRoute;

use super::super::application::service::RenderingService;

#[derive(Debug, Clone)]
pub struct State {
    pub rendering_service: RenderingService,
    pub login_route: WebRoute,
}
