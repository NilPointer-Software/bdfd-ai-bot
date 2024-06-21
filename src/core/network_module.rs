use crate::core::app_config::AppConfigService;
use serenity::client;
use serenity::prelude::GatewayIntents;
use shaku::{Component, Interface};
use std::sync::Arc;

pub trait NetworkModule: Interface {
    fn get_discord_client(&self) -> client::ClientBuilder;
}

#[derive(Component)]
#[shaku(interface = NetworkModule)]
pub struct NetworkModuleImpl {
    #[shaku(inject)]
    app_config: Arc<dyn AppConfigService>,
}

impl NetworkModule for NetworkModuleImpl {
    fn get_discord_client(&self) -> client::ClientBuilder {
        let bot_token = &self.app_config.get().discord_bot_token;
        let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

        serenity::Client::builder(bot_token, intents)
    }
}
