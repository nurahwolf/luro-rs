use luro_framework::{CommandInteraction, InteractionTrait, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::component::TextInputStyle, http::interaction::InteractionResponseType};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "create", desc = "Create a character profile.")]
pub struct Create {
    #[command(desc = "The character to create or modify", autocomplete = true)]
    pub name: String,
}

impl LuroCommand for Create {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let user = ctx.fetch_user(&ctx.author_id()).await?;
        let character = user.fetch_character(&self.name).await?;

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
                                if let Some(character) = &character {
                                    text.value(character.sfw_summary.clone());
                                }
                                text.custom_id("character-sfw-summary")
                                    .label("SFW Summary")
                                    .max_length(250)
                                    .placeholder("An arctic wolf known as the leader of the pack")
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                if let Some(character) = &character {
                                    text.value(character.sfw_summary.clone());
                                }
                                text.custom_id("character-nsfw-summary")
                                    .label("NSFW Summary")
                                    .max_length(250)
                                    .placeholder("Always horny. Going to plow you.")
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                if let Some(character) = &character {
                                    text.value(character.sfw_description.clone());
                                }
                                text.custom_id("character-sfw-description")
                                    .label("SFW Description")
                                    .placeholder("Go absolutely wild here! Write to your hearts content")
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                if let Some(character) = &character {
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
