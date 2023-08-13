use anyhow::{anyhow, Context, Error};

use luro_model::heck::Heck;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    channel::message::{
        component::{ActionRow, TextInput, TextInputStyle},
        Component
    },
    http::interaction::InteractionResponseType
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{interaction::LuroSlash, traits::luro_command::LuroCommand};

#[cfg(not(feature = "toml-driver"))]
fn format_heck_id(input: usize) -> usize {
    input
}

#[cfg(feature = "toml-driver")]
fn format_heck_id(input: usize) -> String {
    input.to_string()
}

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "add", desc = "Add a heck", dm_permission = true)]
pub struct HeckAddCommand {}

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

        ctx.respond(|r| {
            r.title("Write your heck below!")
                .custom_id("heck-add")
                .add_components(components)
                .response_type(InteractionResponseType::Modal)
        })
        .await
    }

    async fn handle_component(self, data: Box<MessageComponentInteractionData>, ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction = &ctx.interaction;
        let interaction_author = interaction.author().context("Expected to get interaction author")?;
        let interaction_channel = interaction.channel.clone().unwrap();
        let nsfw = interaction_channel.nsfw.unwrap_or(false);

        let heck_id;
        let mut field = vec![];

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
            .ok_or_else(|| Error::msg("Unable to find the original heck embed"))?
            .clone();
        let mut heck_author = heck_embed
            .clone()
            .author
            .ok_or_else(|| Error::msg("No author in our heck embed"))?;

        // Create our heck based on the data we have received
        let heck = Heck {
            heck_message: heck_embed
                .clone()
                .description
                .ok_or_else(|| Error::msg("Could not find the new heck in the embed"))?,
            author_id: interaction_author.id
        };

        // Based on our component data, should this be added as a global heck or a guild heck?
        if global.contains("heck-add-global") {
            if nsfw {
                heck_id = ctx.framework.database.nsfw_hecks.len() + 1;
                ctx.framework.database.modify_heck(heck_id, &heck, true).await?;
                ctx.framework.database.nsfw_hecks.insert(format_heck_id(heck_id), heck);
                heck_author.name = "Global Heck Created - NSFW Heck".to_owned();
            } else {
                heck_id = ctx.framework.database.sfw_hecks.len() + 1;
                ctx.framework.database.modify_heck(heck_id, &heck, false).await?;
                ctx.framework.database.sfw_hecks.insert(format_heck_id(heck_id), heck);
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

            let guild_settings = ctx.framework.database.get_guild(&guild_id).await?;

            if nsfw {
                heck_id = guild_settings.nsfw_hecks.len();
                guild_settings.nsfw_hecks.insert(format_heck_id(heck_id), heck);
                ctx.framework.database.update_guild(guild_id, &guild_settings).await?;

                heck_author.name = "Guild Heck Created - NSFW Heck".to_owned()
            } else {
                heck_id = guild_settings.sfw_hecks.len();
                guild_settings.sfw_hecks.insert(format_heck_id(heck_id), heck);
                ctx.framework.database.update_guild(guild_id, &guild_settings).await?;

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
        ctx.respond(|response| response.add_embed(heck_embed).components(|c| c).update())
            .await
    }
}
