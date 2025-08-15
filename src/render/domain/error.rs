use web_route::ParameterizedRoute;

#[derive(Debug, thiserror::Error)]
pub enum RenderDomainError {
    #[error("unable to populate parameterized route: {route}")]
    InvalidRoutePopulation { route: ParameterizedRoute },
}
