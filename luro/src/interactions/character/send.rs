use anyhow::Context;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand, models::LuroWebhook};

#[derive(CommandModel, CreateCommand)]
#[command(name = "send", desc = "Send a message as a character!")]
pub struct SendCommand {
    /// The fursona that should be proxied
    pub name: String,
    /// The message to send
    pub message: String
}

impl LuroCommand for SendCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let user_id = ctx
            .interaction
            .author_id()
            .context("Expected to find the user running this command")?;

        let user_data = ctx.framework.database.get_user(&user_id).await?;
        if user_data.characters.is_empty() {
            return ctx
                .respond(|r| {
                    r.content(format!("Sorry, <@{user_id}> has no character profiles configured!"))
                        .ephemeral()
                })
                .await;
        }

        let character = match user_data.characters.get(&self.name) {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };
        let character_icon = match !character.icon.is_empty() {
            true => character.icon.clone(),
            false => user_data.avatar()
        };

        let luro_webhook = LuroWebhook::new(ctx.framework.clone());
        let webhook = luro_webhook.get_webhook(ctx.interaction.clone().channel.unwrap().id).await?;
        let webhook_token = match webhook.token {
            Some(token) => token,
            None => match ctx.framework.twilight_client.webhook(webhook.id).await?.model().await?.token {
                Some(token) => token,
                None => {
                    return ctx
                        .respond(|r| r.content("I cannot create a webhook here! Sorry!").ephemeral())
                        .await
                }
            }
        };

        ctx.framework
            .twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&format!(
                "{} [{}]",
                self.name,
                user_data.member_name(&ctx.interaction.guild_id)
            ))
            .content(&self.message)
            .avatar_url(&character_icon)
            .await?;

        ctx.respond(|r| r.content("Mirrored!").ephemeral()).await
    }
}
