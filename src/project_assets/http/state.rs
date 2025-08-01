use super::super::application::service::ProjectAssetService;

#[derive(Debug, Clone)]
pub struct State {
    pub project_asset_service: ProjectAssetService,
}
