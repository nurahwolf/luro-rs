use luro_framework::{LuroCommand, InteractionTrait, CommandInteraction, Luro};
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
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let user_id = ctx.author_id();

        let user_data = ctx.get_user(&user_id).await?;
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
            false => user_data.avatar(),
        };

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
            .username(&format!("{} [{}]", self.name, user_data.member_name(&ctx.guild_id)))
            .content(&self.message)
            .avatar_url(&character_icon)
            .await?;

        ctx.respond(|r| r.content("Mirrored!").ephemeral()).await
    }
}
