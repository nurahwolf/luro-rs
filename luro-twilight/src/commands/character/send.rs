use luro_framework::command::LuroCommandTrait;
use luro_framework::Framework;
use luro_framework::InteractionCommand;
use luro_framework::LuroInteraction;
use luro_model::database::drivers::LuroDatabaseDriver;
use std::fmt::Write;

use twilight_interactions::command::AutocompleteValue;
use twilight_interactions::command::CommandModel;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::command::{CommandOptionChoice, CommandOptionChoiceValue},
    http::interaction::InteractionResponseType,
};

#[derive(CommandModel,)]
#[command(autocomplete = true)]
pub struct CharacterSendAutocomplete {
    name: AutocompleteValue<String,>,
}

impl CharacterSendAutocomplete {
    pub async fn interaction_command<D: LuroDatabaseDriver,>(
        self,
        ctx: Framework<D,>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<(),> {
        let user_id = interaction.author_id();
        let user_data = ctx.database.get_user(&user_id, false,).await?;
        let choices = match self.name {
            AutocompleteValue::None => user_data
                .characters
                .keys()
                .map(|name| CommandOptionChoice {
                    name: name.clone(),
                    name_localizations: None,
                    value: CommandOptionChoiceValue::String(name.clone(),),
                },)
                .collect(),
            AutocompleteValue::Focused(input,) => user_data
                .characters
                .keys()
                .filter_map(|name| match name.contains(&input,) {
                    true => Some(CommandOptionChoice {
                        name: name.clone(),
                        name_localizations: None,
                        value: CommandOptionChoiceValue::String(name.clone(),),
                    },),
                    false => None,
                },)
                .collect(),
            AutocompleteValue::Completed(_,) => vec![],
        };

        interaction
            .respond(&ctx, |response| {
                response
                    .choices(choices.into_iter(),)
                    .response_type(InteractionResponseType::ApplicationCommandAutocompleteResult,)
            },)
            .await
    }
}

#[derive(CommandModel, CreateCommand,)]
#[command(name = "send", desc = "Send a message as a character!")]
pub struct CharacterSend {
    #[command(desc = "The character that should be proxied", autocomplete = true)]
    name: String,
    /// The message to send
    message: String,
}
#[async_trait::async_trait]

impl LuroCommandTrait for CharacterSend {
    async fn handle_interaction<D: LuroDatabaseDriver,>(
        ctx: Framework<D,>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<(),> {
        let data = Self::new(interaction.data.clone(),)?;
        let user_id = interaction.author_id();

        let user_data = ctx.database.get_user(&user_id, false,).await?;
        if user_data.characters.is_empty() {
            return interaction
                .respond(&ctx, |r| {
                    r.content(format!("Sorry, <@{user_id}> has no character profiles configured!"),)
                        .ephemeral()
                },)
                .await;
        }

        let character = match user_data.characters.get(&data.name,) {
            Some(character,) => character,
            None => {
                let mut characters = String::new();

                for (character_name, character,) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", data.name, characters);
                return interaction.respond(&ctx, |r| r.content(response,).ephemeral(),).await;
            }
        };
        let character_icon = match !character.icon.is_empty() {
            true => character.icon.clone(),
            false => user_data.avatar(),
        };

        let webhook = ctx.get_webhook(interaction.clone().channel.unwrap().id,).await?;
        let webhook_token = match webhook.token {
            Some(token,) => token,
            None => match ctx.twilight_client.webhook(webhook.id,).await?.model().await?.token {
                Some(token,) => token,
                None => {
                    return interaction
                        .respond(&ctx, |r| r.content("I cannot create a webhook here! Sorry!",).ephemeral(),)
                        .await
                }
            },
        };

        ctx.twilight_client
            .execute_webhook(webhook.id, &webhook_token,)
            .username(&format!("{} [{}]", data.name, user_data.member_name(&interaction.guild_id)),)
            .content(&data.message,)
            .avatar_url(&character_icon,)
            .await?;

        interaction.respond(&ctx, |r| r.content("Mirrored!",).ephemeral(),).await
    }
}
