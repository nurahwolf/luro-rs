use anyhow::Error;
use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
    channel::message::{
        component::{ActionRow, SelectMenu, SelectMenuOption, TextInput, TextInputStyle},
        Component
    }
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, ImageSource};

use crate::{
    commands::LuroCommand,
    interactions::InteractionResponse,
    models::{Heck, LuroResponse},
    responses::invalid_heck,
    LuroContext, SlashResponse, ACCENT_COLOUR
};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "add", desc = "Add a heck", dm_permission = true)]
pub struct HeckAddCommand {}

#[async_trait]
impl LuroCommand for HeckAddCommand {
    async fn run_command(self, _interaction: Interaction, _ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        Ok(heck_modal())
    }

    async fn handle(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let (interaction_channel, interaction_author, _) = self.interaction_context(&interaction, "heck user")?;
        // Get interaction data
        let data = self.parse_component_data(&mut interaction.clone())?;
        let global = data
            .values
            .first()
            .ok_or_else(|| Error::msg("Unable to find interaction data"))?;
        // Get the message of the interaction, to grab out data from it
        let message = interaction
            .message
            .clone()
            .ok_or_else(|| Error::msg("Unable to find the original message"))?;
        // Now get both the embed, and components from the message
        let mut heck_embed = message
            .embeds
            .first()
            .ok_or_else(|| Error::msg("Unable to find the original message"))?
            .clone();
        let mut heck_author = heck_embed
            .clone()
            .author
            .ok_or_else(|| Error::msg("No author in our heck embed"))?;
        let components = message.components;

        // Create our heck based on the data we have received
        let mut heck = vec![Heck {
            heck_message: heck_embed
                .clone()
                .description
                .ok_or_else(|| Error::msg("Could not find the new heck in the embed"))?,
            author_id: interaction_author.id.get()
        }];

        // Based on our component data, should this be added as a global heck or a guild heck?
        if global.contains("heck-add-global") {
            let mut heck_db = ctx.global_data.write();

            if interaction_channel.nsfw.unwrap_or(false) {
                heck_db.hecks.nsfw_hecks.append(&mut heck);
                heck_author.name = "Global Heck Created - NSFW Heck".to_owned()
            } else {
                heck_db.hecks.sfw_hecks.append(&mut heck);
                heck_author.name = "Global Heck Created - SFW Heck".to_owned()
            };
        } else {
            let mut guild_db = ctx.guild_data.write();
            let heck_db = guild_db.entry(
                interaction
                    .guild_id
                    .ok_or_else(|| Error::msg("This place is not a guild. You can only use this option in a guild."))?
            );

            if interaction_channel.nsfw.unwrap_or(false) {
                heck_db.and_modify(|guild| guild.hecks.nsfw_hecks.append(&mut heck));
                heck_author.name = "Guild Heck Created - NSFW Heck".to_owned()
            } else {
                heck_db.and_modify(|guild| guild.hecks.sfw_hecks.append(&mut heck));
                heck_author.name = "Guild Heck Created - SFW Heck".to_owned()
            };
        };

        // Update the heck embed to state that the heck has been created
        heck_embed.author = Some(heck_author);

        // Finally, repond with an updated message
        Ok(InteractionResponse::Update {
            content: None,
            embeds: Some(vec![heck_embed]),
            components: Some(components),
            ephemeral: false
        })
    }

    async fn handle_model(self, interaction: Interaction) -> SlashResponse {
        let luro_response = LuroResponse {
            ephemeral: true,
            deferred: false
        };
        let (_, interaction_author, interaction_member) = self.interaction_context(&interaction, "heck add")?;
        let author_avatar = self.get_partial_member_avatar(interaction_member, &interaction.guild_id, interaction_author);
        let data = self.parse_modal_data(&mut interaction.clone())?;
        let heck_text = self.parse_modal_field_required(&data, "heck-text")?;

        // Make sure heck_text contains both <user> and <author>, else exit early
        match (heck_text.contains("<user>"), heck_text.contains("<author>")) {
            (true, true) => (),
            (true, false) => return Ok(invalid_heck::invalid_heck_response(true, false, heck_text, luro_response)),
            (false, true) => return Ok(invalid_heck::invalid_heck_response(false, true, heck_text, luro_response)),
            (false, false) => return Ok(invalid_heck::invalid_heck_response(true, true, heck_text, luro_response))
        };

        // Send a success message.
        let embed_author = EmbedAuthorBuilder::new(format!("Brand new heck by {}", interaction_author.name))
            .icon_url(ImageSource::url(author_avatar)?)
            .build();
        let embed = EmbedBuilder::new()
            .color(ACCENT_COLOUR)
            .description(heck_text)
            .author(embed_author);

        let components = vec![Component::ActionRow(ActionRow {
            components: vec![Component::SelectMenu(SelectMenu {
                custom_id: "heck-setting".to_owned(),
                disabled: false,
                max_values: None,
                min_values: None,
                options: vec![
                    SelectMenuOption {
                        default: false,
                        description: Some("Can only be used in this guild".to_owned()),
                        emoji: None,
                        label: "Guild Specific Heck".to_owned(),
                        value: "heck-add-guild".to_owned()
                    },
                    SelectMenuOption {
                        default: false,
                        description: Some("Can be used globally, including DMs and other servers".to_owned()),
                        emoji: None,
                        label: "Global Heck".to_owned(),
                        value: "heck-add-global".to_owned()
                    },
                ],
                placeholder: Some("Choose if this is a global or guild specific heck".to_owned())
            })]
        })];

        Ok(InteractionResponse::EmbedComponents {
            embeds: vec![embed.build()],
            components,
            luro_response
        })
    }
}

/// Modal that asks the user to enter a reason for the kick.
///
/// This modal is only shown if the user has not specified a reason in the
/// initial command.
fn heck_modal() -> InteractionResponse {
    let components = vec![Component::ActionRow(ActionRow {
        components: vec![Component::TextInput(TextInput {
            custom_id: "heck-text".to_owned(),
            label: "Enter your new heck below".to_owned(),
            max_length: Some(2048),
            min_length: Some(20),
            placeholder: Some("<author> just gave <user> headpats!!".to_owned()),
            required: Some(true),
            style: TextInputStyle::Paragraph,
            value: None
        })]
    })];

    InteractionResponse::Modal {
        custom_id: "heck-add".to_owned(),
        title: "Write your heck below!".to_owned(),
        components
    }
}
