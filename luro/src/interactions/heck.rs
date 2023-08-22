use anyhow::Error;

use luro_model::{database::drivers::LuroDatabaseDriver, heck::Heck, PRIMARY_BOT_OWNER};
use rand::Rng;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    application::interaction::modal::ModalInteractionData,
    channel::message::{
        component::{ActionRow, SelectMenu, SelectMenuOption, SelectMenuType},
        Component
    },
    id::{marker::GuildMarker, Id},
    user::User
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder};

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
    Info(HeckInfo)
}

impl LuroCommand for HeckCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Add(command) => command.run_command(ctx).await,
            Self::Someone(command) => command.run_command(ctx).await,
            Self::Info(command) => command.run_command(ctx).await
        }
    }

    async fn handle_model<D: LuroDatabaseDriver>(data: ModalInteractionData, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let luro_user = ctx.get_interaction_author(&ctx.interaction).await?;
        let heck_text = ctx.parse_modal_field_required(&data, "heck-text")?;

        match (heck_text.contains("<user>"), heck_text.contains("<author>")) {
            (true, true) => (),
            (true, false) => return ctx.invalid_heck_response(true, false, heck_text).await,
            (false, true) => return ctx.invalid_heck_response(false, true, heck_text).await,
            (false, false) => return ctx.invalid_heck_response(false, false, heck_text).await
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
                        value: "heck-add-guild".to_owned()
                    },
                    SelectMenuOption {
                        default: false,
                        description: Some("Can be used globally, including DMs and other servers".to_owned()),
                        emoji: None,
                        label: "Global Heck".to_owned(),
                        value: "heck-add-global".to_owned()
                    },
                ]),
                placeholder: Some("Choose if this is a global or guild specific heck".to_owned()),
                channel_types: None,
                kind: SelectMenuType::Text
            })]
        })];

        ctx.respond(|response| {
            response.components = Some(components);
            response.add_embed(embed.build())
        })
        .await
    }
}

/// Open the database as writeable and remove a NSFW heck from it, returning the heck removed
async fn get_heck<D: LuroDatabaseDriver>(
    ctx: &LuroSlash<D>,
    id: Option<i64>,
    guild_id: Option<Id<GuildMarker>>,
    global: bool,
    nsfw: bool
) -> anyhow::Result<(Heck, usize)> {
    // A heck type to send if there are no hecks of the type requested!
    let no_heck = Heck {
        heck_message: "No hecks of the requested type found!".to_string(),
        author_id: PRIMARY_BOT_OWNER
    };

    let mut heck_id = match id {
        Some(requested_id) => usize::try_from(requested_id)?,
        None => 0
    };

    Ok(match global {
        true => {
            let hecks = match nsfw {
                true => ctx.framework.database.get_hecks(true).await?,
                false => ctx.framework.database.get_hecks(false).await?
            };

            if heck_id == 0 {
                if hecks.is_empty() {
                    return Ok((no_heck, 69));
                }
                heck_id = rand::thread_rng().gen_range(0..hecks.len())
            }

            let heck = match hecks.get(&heck_id) {
                Some(heck) => (heck.clone(), heck_id),
                None => (no_heck, 69)
            };

            heck
        }
        false => {
            let guild_id =
                guild_id.ok_or_else(|| Error::msg("Guild ID is not present. You can only use this option in a guild."))?;
            let guild_settings = ctx
                .framework
                .database
                .get_guild(&guild_id, &ctx.framework.twilight_client)
                .await?;

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
                    }
                )
            }

            let heck = match nsfw {
                true => guild_settings.nsfw_hecks.get(&heck_id),
                false => guild_settings.sfw_hecks.get(&heck_id)
            };

            match heck {
                Some(heck) => (heck.clone(), heck_id),
                None => (no_heck, 69)
            }
        }
    })
}

/// Replace <user> with <@hecked_user> and <author> with the caller of the heck command
async fn format_heck(heck: &Heck, heck_author: &User, hecked_user: &User) -> Heck {
    Heck {
        heck_message: heck
            .heck_message
            .replace("<user>", &format!("<@{}>", &hecked_user.id))
            .replace("<author>", &format!("<@{}>", &heck_author.id)),
        author_id: heck.author_id
    }
}
