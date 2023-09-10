use anyhow::{anyhow, Context, Error};

use luro_model::{database::drivers::LuroDatabaseDriver, heck::Heck};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData, http::interaction::InteractionResponseType,
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq,)]
#[command(name = "add", desc = "Add a heck", dm_permission = true)]
pub struct HeckAddCommand {}

impl LuroCommand for HeckAddCommand {
    /// Modal that asks the user to enter a reason for the kick.
    ///
    /// This modal is only shown if the user has not specified a reason in the
    /// initial command.
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        ctx.respond(|r| {
            r.title("Write your heck below!",)
                .custom_id("heck-add",)
                .components(|components| {
                    components.action_row(|row| {
                        row.text_input(|input| {
                            input
                                .custom_id("heck-text",)
                                .label("Enter your new heck below",)
                                .min_length(20,)
                                .placeholder("<author> just gave <user> headpats!!",)
                        },)
                    },)
                },)
                .response_type(InteractionResponseType::Modal,)
        },)
            .await
    }

    async fn handle_component<D: LuroDatabaseDriver,>(
        self,
        data: Box<MessageComponentInteractionData,>,
        ctx: LuroSlash<D,>,
    ) -> anyhow::Result<(),> {
        let interaction = &ctx.interaction;
        let interaction_author = interaction.author().context("Expected to get interaction author",)?;
        let interaction_channel = interaction.channel.clone().unwrap();
        let nsfw = interaction_channel.nsfw.unwrap_or(false,);

        let heck_id;
        let mut field = vec![];

        // Get interaction data
        // TODO: Don't get data in the command
        let global = data
            .values
            .first()
            .ok_or_else(|| Error::msg("Unable to find interaction data",),)?;
        // Get the message of the interaction, to grab out data from it
        let message = ctx
            .interaction
            .message
            .clone()
            .ok_or_else(|| Error::msg("Unable to find the original message",),)?;
        // Now get both the embed, and components from the message
        let mut heck_embed = message
            .embeds
            .first()
            .ok_or_else(|| Error::msg("Unable to find the original heck embed",),)?
            .clone();
        let mut heck_author = heck_embed
            .clone()
            .author
            .ok_or_else(|| Error::msg("No author in our heck embed",),)?;

        // Create our heck based on the data we have received
        let heck = Heck {
            heck_message: heck_embed
                .clone()
                .description
                .ok_or_else(|| Error::msg("Could not find the new heck in the embed",),)?,
            author_id: interaction_author.id,
        };

        // Based on our component data, should this be added as a global heck or a guild heck?
        if global.contains("heck-add-global",) {
            let hecks = ctx.framework.database.get_hecks(nsfw,).await?;
            heck_id = hecks.len() + 1;
            if nsfw {
                ctx.framework.database.save_heck(heck_id, heck, true,).await?;
                heck_author.name = "Global Heck Created - NSFW Heck".to_owned();
            } else {
                ctx.framework.database.save_heck(heck_id, heck, false,).await?;
                heck_author.name = "Global Heck Created - SFW Heck".to_owned();
            };
            field.append(&mut vec![EmbedFieldBuilder::new("Global Heck", "Just created",)
                .inline()
                .build()],);
        } else {
            let guild_id = match ctx.interaction.guild_id {
                Some(guild_id,) => guild_id,
                None => return Err(anyhow!("This place is not a guild. You can only use this option in a guild."),),
            };

            let mut guild_settings = ctx.framework.database.get_guild(&guild_id,).await?;

            if nsfw {
                heck_id = guild_settings.nsfw_hecks.len();
                guild_settings.nsfw_hecks.insert(heck_id, heck,);
                ctx.framework.database.save_guild(&guild_id, &guild_settings,).await?;

                heck_author.name = "Guild Heck Created - NSFW Heck".to_owned()
            } else {
                heck_id = guild_settings.sfw_hecks.len();
                guild_settings.sfw_hecks.insert(heck_id, heck,);
                ctx.framework.database.save_guild(&guild_id, &guild_settings,).await?;

                heck_author.name = "Guild Heck Created - SFW Heck".to_owned()
            };
            field.append(&mut vec![EmbedFieldBuilder::new("Guild Heck", "Just created",)
                .inline()
                .build()],);
        };
        field.append(&mut vec![EmbedFieldBuilder::new("Heck ID", heck_id.to_string(),)
            .inline()
            .build()],);

        // Add our fields
        heck_embed.fields.append(&mut field,);

        // Update the heck embed to state that the heck has been created
        heck_embed.author = Some(heck_author,);

        // Finally, repond with an updated message. The strips out the previous components
        ctx.respond(|response| response.add_embed(heck_embed,).components(|c| c,).update(),)
            .await
    }
}
