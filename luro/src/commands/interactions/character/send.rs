use std::fmt::Write;

use twilight_interactions::command::CommandModel;
use twilight_interactions::command::CreateCommand;

use crate::models::interaction::{InteractionContext, InteractionResult};

const DELAY_TIME: u64 = 10;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "send", desc = "Send a message as a character!")]
pub struct Command {
    #[command(desc = "The character that should be proxied")]
    pub name: String,
    /// The message to send
    message: String,
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        let character = match ctx.database.user_fetch_character(ctx.author.user_id, &self.name).await? {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for character in ctx.database.user_fetch_characters(ctx.author.user_id).await? {
                    writeln!(characters, "- {}: {}", character.name, character.sfw_summary)?
                }

                let response = format!("I'm afraid that user <@{}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}",ctx.author.user_id, self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        let character_icon = match ctx.channel.nsfw.unwrap_or_default() {
            true => character.nsfw_icon.unwrap_or(character.sfw_icon),
            false => character.sfw_icon,
        };

        // Attempt to first send as a webhook
        if let Ok(webhook) = ctx.get_webhook(ctx.channel.id).await {
            if let Some(token) = webhook.token {
                let response = ctx
                    .twilight_client()
                    .execute_webhook(webhook.id, &token)
                    .username(&format!("{} [{}]", character.name, ctx.author.name()))
                    .content(&self.message)
                    .avatar_url(&character_icon)
                    .await;

                if response.is_ok() {
                    ctx.respond(|r| r.content("Mirrored!").ephemeral()).await?;
                    tokio::time::sleep(tokio::time::Duration::from_secs(DELAY_TIME)).await;
                    ctx.interaction_client().delete_response(&ctx.interaction_token).await?;
                    return Ok(luro_model::types::CommandResponse::default());
                }
            }
        }

        // That failed, so send via the bot as an embed
        ctx.respond(|r| {
            r.embed(|e| {
                e.author(|a| {
                    a.name(format!("{} [{}]", character.nickname.unwrap_or(character.name), ctx.author.name()))
                        .icon_url(character_icon)
                })
                .colour(character.colour.unwrap_or(ctx.accent_colour()))
                .description(self.message)
            })
        })
        .await
    }
}
