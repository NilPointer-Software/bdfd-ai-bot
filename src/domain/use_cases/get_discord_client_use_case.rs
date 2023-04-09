use std::sync::Arc;
use shaku::{Component, Interface};
use crate::core::network_module::{NetworkModule};
use async_trait::async_trait;

#[async_trait]
pub trait GetDiscordClientUseCase: Interface {
    async fn call(&self) -> serenity::client::ClientBuilder;
}

#[derive(Component)]
#[shaku(interface = GetDiscordClientUseCase)]
pub struct GetDiscordClientUseCaseImpl {
    #[shaku(inject)]
    network_module: Arc<dyn NetworkModule>,
}

#[async_trait]
impl GetDiscordClientUseCase for GetDiscordClientUseCaseImpl {
    async fn call(&self) -> serenity::client::ClientBuilder {
        self.network_module.get_discord_client()
    }
}