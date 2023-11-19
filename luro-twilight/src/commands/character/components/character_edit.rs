use anyhow::Context;
use luro_framework::ComponentInteraction;
use luro_model::types::CommandResponse;
use twilight_model::{
    application::interaction::Interaction, channel::message::component::TextInputStyle, http::interaction::InteractionResponseType,
};

impl crate::commands::character::Character {
    pub async fn character_edit_button(
        &self,
        ctx: ComponentInteraction,
        invoking_interaction: Interaction,
    ) -> anyhow::Result<CommandResponse> {
        let original_author_id = invoking_interaction
            .author_id()
            .context("Expected to get user ID from interaction")?;
        if ctx.author.user_id != original_author_id {
            return ctx
                .respond(|r| {
                    r.content(format!("Sorry, only the profile owner <@{original_author_id}> can edit a profile!"))
                        .ephemeral()
                })
                .await;
        }
        let character_name = self.character_name();
        let character = match ctx.database.user_fetch_character(original_author_id, character_name).await? {
            Some(character) => character,
            None => return ctx.respond(|r|r.content(format!("Sorry, could not find the character {character_name} in my database. The user might have deleted this profile, sorry!")).ephemeral()).await,
        };

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
                                    .value(&character.name)
                                    .style(TextInputStyle::Short)
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                text.value(character.sfw_summary.clone())
                                    .custom_id("character-sfw-summary")
                                    .label("SFW Summary")
                                    .max_length(250)
                                    .placeholder("An arctic wolf known as the leader of the pack")
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                if let Some(nsfw_summary) = &character.nsfw_summary {
                                    text.value(nsfw_summary.clone());
                                }
                                text.custom_id("character-nsfw-summary")
                                    .label("NSFW Summary")
                                    .max_length(250)
                                    .placeholder("Always horny. Going to plow you.")
                                    .required(false)
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                text.value(character.sfw_description.clone())
                                    .custom_id("character-sfw-description")
                                    .label("SFW Description")
                                    .placeholder("Go absolutely wild here! Write to your hearts content")
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                if let Some(nsfw_description) = &character.nsfw_description {
                                    text.value(nsfw_description);
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
