use anyhow::Context;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::component::TextInputStyle, http::interaction::InteractionResponseType};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create a character profile.")]
pub struct Create {
    #[command(desc = "The character to create or modify", autocomplete = true)]
    pub name: String
}

impl LuroCommand for Create {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let user_id = ctx
            .interaction
            .author_id()
            .context("Expected to find the user running this command")?;
        let user_data = ctx.framework.database.get_user(&user_id, &ctx.framework.twilight_client).await?;
        let character = user_data.characters.get(&self.name);

        // Create a model
        ctx.respond(|r| {
            r.title("Create or modify a Character!")
                .components(|components| {
                    components
                        .action_row(|row| {
                            row.text_input(|text| {
                                text.custom_id("character-name")
                                    .label("Character Name")
                                    .max_length(40)
                                    .placeholder("Nurah")
                                    .value(&self.name)
                                    .style(TextInputStyle::Short)
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                if let Some(character) = character {
                                    text.value(&character.short_description);
                                }
                                text.custom_id("character-short-description")
                                    .label("Short Description")
                                    .max_length(250)
                                    .placeholder("An arctic wolf known as the leader of the pack")
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                if let Some(character) = character {
                                    text.value(&character.description);
                                }
                                text.custom_id("character-description")
                                    .label("Long Description")
                                    .placeholder("Go absolutely wild here! Write to your hearts content")
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                if let Some(character) = character {
                                    if let Some(description) = &character.nsfw_description {
                                        text.value(description);
                                    }
                                }
                                text.custom_id("character-nsfw-description")
                                    .label("NSFW Description")
                                    .placeholder("Optional. Shows SFW description if not set.")
                                    .required(false)
                            })
                        })
                })
                .custom_id("character")
                .response_type(InteractionResponseType::Modal)
        })
        .await
    }
}
