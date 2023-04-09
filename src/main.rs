use flexi_logger::Logger;
use shaku::{HasComponent, module};

use crate::core::app_config::AppConfigServiceImpl;
use crate::core::network_module::NetworkModuleImpl;
use crate::data::ai::ai_repository::AIRepositoryImpl;
use crate::domain::ai::use_cases::generate_first_message_help_use_case::GenerateFirstMessageHelpUseCaseImpl;
use crate::domain::use_cases::get_app_config_use_case::GetAppConfigUseCaseImpl;
use crate::domain::use_cases::get_discord_client_use_case::GetDiscordClientUseCaseImpl;
use crate::infrastructure::discord_bot::{DiscordBot, DiscordBotImpl};

mod core;
mod data;
mod domain;
mod infrastructure;

module! {
    MainModule {
        components = [
            AppConfigServiceImpl,
            AIRepositoryImpl,
            NetworkModuleImpl,
            GenerateFirstMessageHelpUseCaseImpl,
            GetDiscordClientUseCaseImpl,
            GetAppConfigUseCaseImpl,
            DiscordBotImpl,
        ],
        providers = []
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::try_with_env_or_str("info, tracing::span=warn").unwrap().start().unwrap();

    let module = MainModule::builder().build();
    let discord_bot: &dyn DiscordBot = module.resolve_ref();
    discord_bot.start_bot().await?;

    Ok(())
}
