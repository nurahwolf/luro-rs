use luro_framework::{CommandInteraction, LuroCommand};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Write;

use twilight_interactions::command::CommandModel;
use twilight_interactions::command::CreateCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "send", desc = "Send a message as a character!")]
pub struct CharacterSend {
    #[command(desc = "The character that should be proxied", autocomplete = true)]
    name: String,
    /// The message to send
    message: String,
}

impl LuroCommand for CharacterSend {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let character = match ctx.author.fetch_character(ctx.database.clone(), &self.name).await? {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for (character_name, character) in ctx.author.fetch_characters(ctx.database.clone()).await? {
                    writeln!(characters, "- {character_name}: {}", character.sfw_summary)?
                }

                let response = format!("I'm afraid that user <@{}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}",ctx.author.user_id, self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        let character_icon = character
            .sfw_icons
            .map(|x| x.choose(&mut thread_rng()).cloned())
            .map(|x| x.unwrap_or(ctx.author.avatar_url()))
            .unwrap_or(ctx.author.avatar_url());

        let webhook = ctx.get_webhook(ctx.channel.id).await?;
        let webhook_token = match webhook.token {
            Some(token) => token,
            None => match ctx.twilight_client.webhook(webhook.id).await?.model().await?.token {
                Some(token) => token,
                None => {
                    return ctx
                        .respond(|r| r.content("I cannot create a webhook here! Sorry!").ephemeral())
                        .await
                }
            },
        };

        ctx.twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&format!("{} [{}]", self.name, ctx.author.name()))
            .content(&self.message)
            .avatar_url(&character_icon)
            .await?;

        ctx.respond(|r| r.content("Mirrored!").ephemeral()).await
    }
}
