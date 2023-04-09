use std::sync::Arc;
use shaku::{Component, Interface};
use crate::core::app_config::{AppConfig, AppConfigService};

pub trait GetAppConfigUseCase: Interface {
    fn call(&self) -> AppConfig;
}

#[derive(Component)]
#[shaku(interface = GetAppConfigUseCase)]
pub struct GetAppConfigUseCaseImpl {
    #[shaku(inject)]
    app_config_service: Arc<dyn AppConfigService>,
}

impl GetAppConfigUseCase for GetAppConfigUseCaseImpl {
    fn call(&self) -> AppConfig {
        self.app_config_service.get()
    }
}