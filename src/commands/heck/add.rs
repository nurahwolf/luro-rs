use std::convert::TryInto;

use anyhow::{anyhow, Error};
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::{message_component::MessageComponentInteractionData, modal::ModalInteractionData},
    channel::message::{
        component::{ActionRow, SelectMenu, SelectMenuOption, TextInput, TextInputStyle},
        Component
    }
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder};

use crate::{
    models::LuroSlash,
    models::{GuildSetting, Heck},
    traits::{luro_command::LuroCommand, luro_functions::LuroFunctions},
    ACCENT_COLOUR
};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "add", desc = "Add a heck", dm_permission = true)]
pub struct HeckAddCommand {}

#[async_trait]
impl LuroCommand for HeckAddCommand {
    /// Modal that asks the user to enter a reason for the kick.
    ///
    /// This modal is only shown if the user has not specified a reason in the
    /// initial command.
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
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

        ctx.custom_id("heck-add".to_owned())
            .title("Write your heck below!".to_owned())
            .components(components)
            .model()
            .respond()
            .await
    }

    async fn handle_component(data: MessageComponentInteractionData, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let mut heck_id = 0;
        let mut field = vec![];
        let interaction_channel = ctx.channel()?;
        let interaction_author = ctx.author()?;

        // Get interaction data
        // TODO: Don't get data in the command
        let global = data
            .values
            .first()
            .ok_or_else(|| Error::msg("Unable to find interaction data"))?;
        // Get the message of the interaction, to grab out data from it
        let message = ctx
            .interaction
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
            let mut heck_db = ctx.luro.global_data.write();

            if interaction_channel.nsfw.unwrap_or(false) {
                heck_db.hecks.nsfw_hecks.append(&mut heck);
                heck_id = heck_db.hecks.nsfw_hecks.len();
                heck_author.name = "Global Heck Created - NSFW Heck".to_owned();
            } else {
                heck_db.hecks.sfw_hecks.append(&mut heck);
                heck_id = heck_db.hecks.sfw_hecks.len();
                heck_author.name = "Global Heck Created - SFW Heck".to_owned();
            };
            field.append(&mut vec![EmbedFieldBuilder::new("Global Heck", "Just created")
                .inline()
                .build()]);
        } else {
            let guild_id = match ctx.interaction.guild_id {
                Some(guild_id) => guild_id,
                None => return Err(anyhow!("This place is not a guild. You can only use this option in a guild."))
            };

            // Make sure guild settings are present
            GuildSetting::get_guild_settings(&ctx.luro, &guild_id).await?;

            let heck_db = ctx.luro.guild_data.entry(guild_id);

            if interaction_channel.nsfw.unwrap_or(false) {
                heck_db.and_modify(|guild| {
                    heck_id = guild.hecks.nsfw_hecks.len();
                    guild.hecks.sfw_hecks.append(&mut heck)
                });

                heck_author.name = "Guild Heck Created - NSFW Heck".to_owned()
            } else {
                heck_db.and_modify(|guild| {
                    heck_id = guild.hecks.sfw_hecks.len();
                    guild.hecks.sfw_hecks.append(&mut heck)
                });

                heck_author.name = "Guild Heck Created - SFW Heck".to_owned()
            };
            field.append(&mut vec![EmbedFieldBuilder::new("Guild Heck", "Just created")
                .inline()
                .build()]);
        };
        field.append(&mut vec![EmbedFieldBuilder::new("Heck ID", heck_id.to_string())
            .inline()
            .build()]);

        // Add our fields
        heck_embed.fields.append(&mut field);

        // Update the heck embed to state that the heck has been created
        heck_embed.author = Some(heck_author);

        // Finally, repond with an updated message
        ctx.embed(heck_embed)?.components(components).update().respond().await
    }

    async fn handle_model(data: ModalInteractionData, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let (_author, slash_author) = ctx.get_interaction_author(&ctx.interaction)?;
        let heck_text = ctx.parse_modal_field_required(&data, "heck-text")?;

        match (heck_text.contains("<user>"), heck_text.contains("<author>")) {
            (true, true) => (),
            (true, false) => return ctx.invalid_heck_response(true, false, heck_text).await,
            (false, true) => return ctx.invalid_heck_response(false, true, heck_text).await,
            (false, false) => return ctx.invalid_heck_response(false, false, heck_text).await
        };

        // Send a success message.
        let embed_author = EmbedAuthorBuilder::new(format!("Brand new heck by {}", slash_author.name))
            .icon_url(slash_author.try_into()?)
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

        ctx.embed(embed.build())?.components(components).respond().await
    }
}
