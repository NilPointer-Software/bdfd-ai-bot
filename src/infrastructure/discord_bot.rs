use async_trait::async_trait;
use log::{debug, info, trace, warn};
use serenity::http::{CacheHttp, Typing};
use serenity::model::channel::Channel;
use serenity::model::prelude::{Message, PermissionOverwrite, PermissionOverwriteType, Ready};
use serenity::prelude::*;
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::domain::ai::use_cases::generate_first_message_help_use_case::GenerateFirstMessageHelpUseCase;
use crate::domain::use_cases::get_app_config_use_case::GetAppConfigUseCase;
use crate::domain::use_cases::get_discord_client_use_case::GetDiscordClientUseCase;

#[async_trait]
pub trait DiscordBot: Interface {
    async fn start_bot(&self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Component)]
#[shaku(interface = DiscordBot)]
pub struct DiscordBotImpl {
    #[shaku(inject)]
    generate_first_message_help_use_case: Arc<dyn GenerateFirstMessageHelpUseCase>,
    #[shaku(inject)]
    get_discord_client_use_case: Arc<dyn GetDiscordClientUseCase>,
    #[shaku(inject)]
    get_app_config_use_case: Arc<dyn GetAppConfigUseCase>,
}

#[async_trait]
impl DiscordBot for DiscordBotImpl {
    async fn start_bot(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client_builder = self.get_discord_client_use_case.call().await;
        let handler = DiscordMessageHandler {
            generate_first_message_help_use_case: self.generate_first_message_help_use_case.clone(),
            channel_parent_id: self.get_app_config_use_case.call().channel_parent_id,
        };

        let mut client = client_builder.event_handler(handler).await?;
        client.start().await?;

        Ok(())
    }
}

struct DiscordMessageHandler {
    generate_first_message_help_use_case: Arc<dyn GenerateFirstMessageHelpUseCase>,
    channel_parent_id: String,
}

impl DiscordMessageHandler {
    async fn parse_message_with_result(
        &self,
        ctx: Context,
        msg: Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Message received! {}", msg.content);

        if msg.author.bot {
            trace!("Ignoring message due to bot user");
            return Ok(());
        }

        if msg.content.starts_with('!') {
            trace!("Ignoring message due to being a command");
            return Ok(());
        }

        let channel = msg.channel(&ctx.http()).await?;
        let channel = match channel {
            Channel::Guild(guild_channel) => guild_channel,
            _ => {
                trace!("Ignoring message due to not being in the guild channel");
                return Ok(());
            }
        };
        let channel_parent_id = channel.parent_id.ok_or("Missing parent channel id")?;

        if channel_parent_id.to_string() != self.channel_parent_id {
            trace!("Ignoring message due to not matching channel category (parent) id");
            return Ok(());
        }

        let message_author = msg.author.id;
        let member_overwrites: Vec<&PermissionOverwrite> = channel
            .permission_overwrites
            .iter()
            .filter(|x| matches!(x.kind, PermissionOverwriteType::Member(_)))
            .collect();

        let message_check: Box<dyn Fn(&&Message) -> bool + Send> = if !member_overwrites.is_empty()
        {
            let res = member_overwrites
                .iter()
                .find(|x| x.kind == PermissionOverwriteType::Member(message_author));
            if res.is_none() {
                trace!("Ignoring message due to the author not being the creator of the ticket");
                return Ok(());
            }
            Box::new(|msg| msg.author.id == message_author)
        } else {
            Box::new(|msg| !msg.author.bot)
        };

        let current_user_id = ctx.cache.current_user_id();

        let is_first_user_message = !channel
            .messages(&ctx.http, |retriever| retriever.before(msg.id))
            .await?
            .iter()
            .any(|msg| message_check(&msg) || msg.author.id == current_user_id);

        if !is_first_user_message {
            debug!("Ignoring message due to not being first one");
            return Ok(());
        }

        debug!("Generating response");
        let typing = Typing::start(ctx.http.clone(), msg.channel_id.0)?;
        let response_content = self
            .generate_first_message_help_use_case
            .call(msg.content)
            .await?;
        info!("Generated response content!");

        let ai_generated_message_disclaimer =
            include_str!("data/ai_generated_message_disclaimer.txt");
        let response_message = format!("{response_content}\n\n*{ai_generated_message_disclaimer}*");

        msg.channel_id.say(&ctx.http, response_message).await?;
        _ = typing.stop();

        Ok(())
    }
}

#[async_trait]
impl EventHandler for DiscordMessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        let result = self.parse_message_with_result(ctx, msg).await;
        if let Err(err) = result {
            warn!("Failed to process message: {}", err)
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Discord bot ({}) is connected!", ready.user.name);
    }
}
