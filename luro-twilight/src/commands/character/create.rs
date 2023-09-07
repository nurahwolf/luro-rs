use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::component::TextInputStyle, http::interaction::InteractionResponseType};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create a character profile.")]
pub struct Create {
    #[command(desc = "The character to create or modify", autocomplete = true)]
    pub name: String
}

#[async_trait]
impl LuroCommandTrait for Create {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let user_id = interaction.author_id();
        let user_data = ctx.database.get_user(&user_id).await?;
        let character = user_data.characters.get(&data.name);

        // Create a model
        interaction
            .respond(&ctx, |r| {
                r.title("Create or modify a Character!")
                    .components(|components| {
                        components
                            .action_row(|row| {
                                row.text_input(|text| {
                                    text.custom_id("character-name")
                                        .label("Character Name")
                                        .max_length(40)
                                        .placeholder("Nurah")
                                        .value(&data.name)
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
