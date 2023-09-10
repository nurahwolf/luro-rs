use anyhow::Context;
use luro_model::database::drivers::LuroDatabaseDriver;
use std::fmt::Write;
use twilight_interactions::command::AutocompleteValue;
use twilight_interactions::command::CommandModel;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::command::{CommandOptionChoice, CommandOptionChoiceValue},
    http::interaction::InteractionResponseType
};

use crate::{interaction::LuroSlash, luro_command::LuroCommand, models::LuroWebhook};

#[derive(CommandModel)]
#[command(autocomplete = true)]
pub struct CharacterSendAutocomplete {
    name: AutocompleteValue<String>
}

impl CharacterSendAutocomplete {
    pub async fn run<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let user_id = ctx
            .interaction
            .author_id()
            .context("Expected to find the user running this command")?;

        let user_data = ctx.framework.database.get_user(&user_id, false).await?;
        let choices = match self.name {
            AutocompleteValue::None => user_data
                .characters
                .keys()
                .map(|name| CommandOptionChoice {
                    name: name.clone(),
                    name_localizations: None,
                    value: CommandOptionChoiceValue::String(name.clone())
                })
                .collect(),
            AutocompleteValue::Focused(input) => user_data
                .characters
                .keys()
                .filter_map(|name| match name.contains(&input) {
                    true => Some(CommandOptionChoice {
                        name: name.clone(),
                        name_localizations: None,
                        value: CommandOptionChoiceValue::String(name.clone())
                    }),
                    false => None
                })
                .collect(),
            AutocompleteValue::Completed(_) => vec![]
        };

        ctx.respond(|response| {
            response
                .choices(choices.into_iter())
                .response_type(InteractionResponseType::ApplicationCommandAutocompleteResult)
        })
        .await
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "send", desc = "Send a message as a character!")]
pub struct CharacterSend {
    #[command(desc = "The character that should be proxied", autocomplete = true)]
    name: String,
    /// The message to send
    message: String
}

impl LuroCommand for CharacterSend {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let user_id = ctx
            .interaction
            .author_id()
            .context("Expected to find the user running this command")?;

        let user_data = ctx.framework.database.get_user(&user_id, false).await?;
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
