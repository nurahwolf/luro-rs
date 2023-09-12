use anyhow::{anyhow, Context, Error};

use luro_model::{database_driver::LuroDatabaseDriver, heck::Heck, PRIMARY_BOT_OWNER};
use rand::Rng;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    application::interaction::{message_component::MessageComponentInteractionData, modal::ModalInteractionData},
    channel::message::{
        component::{ActionRow, SelectMenu, SelectMenuOption, SelectMenuType},
        Component,
    },
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder};

use crate::{interaction::LuroSlash, luro_command::LuroCommand, ACCENT_COLOUR};

use self::{add::HeckAddCommand, info::HeckInfo, someone::HeckSomeoneCommand};

pub mod add;
mod info;
mod someone;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "heck",
    desc = "Send a playful message at a user of your choice",
    dm_permission = true
)]
pub enum HeckCommands {
    #[command(name = "add")]
    Add(HeckAddCommand),
    #[command(name = "someone")]
    Someone(Box<HeckSomeoneCommand>),
    #[command(name = "info")]
    Info(HeckInfo),
}

impl LuroCommand for HeckCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Add(command) => command.run_command(ctx).await,
            Self::Someone(command) => command.run_command(ctx).await,
            Self::Info(command) => command.run_command(ctx).await,
        }
    }

    async fn handle_model<D: LuroDatabaseDriver>(data: ModalInteractionData, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let luro_user = ctx.get_interaction_author(&ctx.interaction).await?;
        let heck_text = ctx.parse_modal_field_required(&data, "heck-text")?;

        match (heck_text.contains("<user>"), heck_text.contains("<author>")) {
            (true, true) => (),
            (true, false) => return ctx.invalid_heck_response(true, false, heck_text).await,
            (false, true) => return ctx.invalid_heck_response(false, true, heck_text).await,
            (false, false) => return ctx.invalid_heck_response(false, false, heck_text).await,
        };

        // Send a success message.
        let embed_author = EmbedAuthorBuilder::new(format!("Brand new heck by {}", luro_user.name())).build();
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
                options: Some(vec![
                    SelectMenuOption {
                        default: false,
                        description: Some("Can only be used in this guild".to_owned()),
                        emoji: None,
                        label: "Guild Specific Heck".to_owned(),
                        value: "heck-add-guild".to_owned(),
                    },
                    SelectMenuOption {
                        default: false,
                        description: Some("Can be used globally, including DMs and other servers".to_owned()),
                        emoji: None,
                        label: "Global Heck".to_owned(),
                        value: "heck-add-global".to_owned(),
                    },
                ]),
                placeholder: Some("Choose if this is a global or guild specific heck".to_owned()),
                channel_types: None,
                kind: SelectMenuType::Text,
            })],
        })];

        ctx.respond(|response| {
            response.components = Some(components);
            response.add_embed(embed.build())
        })
        .await
    }

    async fn handle_component<D: LuroDatabaseDriver>(
        self,
        data: Box<MessageComponentInteractionData>,
        ctx: LuroSlash<D>,
    ) -> anyhow::Result<()> {
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
            author_id: interaction_author.id,
            nsfw,
        };

        // Based on our component data, should this be added as a global heck or a guild heck?
        if global.contains("heck-add-global") {
            let hecks = ctx.framework.database.get_hecks(nsfw).await?;
            heck_id = hecks.len() + 1;
            if nsfw {
                ctx.framework.database.save_heck(heck_id, heck).await?;
                heck_author.name = "Global Heck Created - NSFW Heck".to_owned();
            } else {
                ctx.framework.database.save_heck(heck_id, heck).await?;
                heck_author.name = "Global Heck Created - SFW Heck".to_owned();
            };
            field.append(&mut vec![EmbedFieldBuilder::new("Global Heck", "Just created")
                .inline()
                .build()]);
        } else {
            let guild_id = match ctx.interaction.guild_id {
                Some(guild_id) => guild_id,
                None => return Err(anyhow!("This place is not a guild. You can only use this option in a guild.")),
            };

            let mut guild_settings = ctx.framework.database.get_guild(&guild_id).await?;

            if nsfw {
                heck_id = guild_settings.nsfw_hecks.len();
                guild_settings.nsfw_hecks.insert(heck_id, heck);
                ctx.framework.database.modify_guild(&guild_id, &guild_settings).await?;

                heck_author.name = "Guild Heck Created - NSFW Heck".to_owned()
            } else {
                heck_id = guild_settings.sfw_hecks.len();
                guild_settings.sfw_hecks.insert(heck_id, heck);
                ctx.framework.database.modify_guild(&guild_id, &guild_settings).await?;

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

        // Finally, repond with an updated message. The strips out the previous components
        ctx.respond(|response| response.add_embed(heck_embed).components(|c| c).update())
            .await
    }
}

/// Open the database as writeable and remove a NSFW heck from it, returning the heck removed
async fn get_heck<D: LuroDatabaseDriver>(
    ctx: &LuroSlash<D>,
    id: Option<i64>,
    guild_id: Option<Id<GuildMarker>>,
    global: bool,
    nsfw: bool,
) -> anyhow::Result<(Heck, usize)> {
    // A heck type to send if there are no hecks of the type requested!
    let no_heck = Heck {
        heck_message: "No hecks of the requested type found!".to_string(),
        author_id: PRIMARY_BOT_OWNER,
        nsfw: false,
    };

    let mut heck_id = match id {
        Some(requested_id) => requested_id as usize,
        None => 0,
    };

    Ok(match global {
        true => {
            let hecks = ctx.framework.database.get_hecks(nsfw).await?;

            if heck_id == 0 {
                if hecks.is_empty() {
                    return Ok((no_heck, 69));
                }
                heck_id = rand::thread_rng().gen_range(0..hecks.len())
            }

            match hecks.get(&heck_id) {
                Some(heck) => (heck.clone(), heck_id),
                None => (no_heck, 69),
            }
        }
        false => {
            let guild_id =
                guild_id.ok_or_else(|| Error::msg("Guild ID is not present. You can only use this option in a guild."))?;
            let guild_settings = ctx.framework.database.get_guild(&guild_id).await?;

            if heck_id == 0 {
                heck_id = rand::thread_rng().gen_range(
                    0..match nsfw {
                        true => {
                            if guild_settings.nsfw_hecks.is_empty() {
                                return Ok((no_heck, 69));
                            }
                            guild_settings.nsfw_hecks.len()
                        }
                        false => {
                            if guild_settings.sfw_hecks.is_empty() {
                                return Ok((no_heck, 69));
                            }
                            guild_settings.sfw_hecks.len()
                        }
                    },
                )
            }

            let heck = match nsfw {
                true => guild_settings.nsfw_hecks.get(&heck_id),
                false => guild_settings.sfw_hecks.get(&heck_id),
            };

            match heck {
                Some(heck) => (heck.clone(), heck_id),
                None => (no_heck, 69),
            }
        }
    })
}

/// Replace <user> with <@hecked_user> and <author> with the caller of the heck command
async fn format_heck(heck: &Heck, heck_author: &Id<UserMarker>, hecked_user: &Id<UserMarker>, nsfw: bool) -> Heck {
    Heck {
        heck_message: heck
            .heck_message
            .replace("<user>", &format!("<@{}>", &hecked_user))
            .replace("<author>", &format!("<@{}>", &heck_author)),
        author_id: heck.author_id,
        nsfw,
    }
}
