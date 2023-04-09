use shaku::{Component, Interface};
use std::env::var_os;

const OPEN_AI_TOKEN_VAR_NAME: &str = "OPEN_AI_TOKEN";
const DISCORD_BOT_TOKEN_VAR_NAME: &str = "DISCORD_BOT_TOKEN";
const CHANNEL_PARENT_ID_VAR_NAME: &str = "CHANNEL_PARENT_ID";

pub trait AppConfigService: Interface {
    fn get(&self) -> AppConfig;
}

#[derive(Component)]
#[shaku(interface = AppConfigService)]
pub struct AppConfigServiceImpl {}

impl AppConfigService for AppConfigServiceImpl {
    fn get(&self) -> AppConfig {
        AppConfig::new_from_env()
    }
}

pub struct AppConfig {
    pub open_ai_token: String,
    pub discord_bot_token: String,
    pub channel_parent_id: String,
}

impl AppConfig {
    pub fn new_from_env() -> Self {
        let open_ai_token = var_os(OPEN_AI_TOKEN_VAR_NAME);
        let discord_bot_token = var_os(DISCORD_BOT_TOKEN_VAR_NAME);
        let channel_parent_id = var_os(CHANNEL_PARENT_ID_VAR_NAME);
        if open_ai_token.is_none() || discord_bot_token.is_none() || channel_parent_id.is_none() {
            panic!("You have to provide token values!");
        }

        let open_ai_token = open_ai_token.unwrap().into_string().unwrap();
        openai::set_key(open_ai_token.to_string());

        AppConfig {
            open_ai_token,
            discord_bot_token: discord_bot_token.unwrap().into_string().unwrap(),
            channel_parent_id: channel_parent_id.unwrap().into_string().unwrap(),
        }
    }
}
