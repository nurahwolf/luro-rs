use std::convert::TryInto;

use anyhow::Error;

use async_trait::async_trait;
use luro_model::{constants::PRIMARY_BOT_OWNER, heck::Heck};
use rand::Rng;

use twilight_interactions::command::{CommandModel, CreateCommand};

use tracing::debug;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    user::User
};

use crate::{slash::Slash, traits::luro_command::LuroCommand, LuroFramework};

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

#[async_trait]
impl LuroCommand for HeckCommands {
    async fn run_commands(self, ctx: Slash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Add(command) => command.run_command(ctx).await,
            Self::Someone(command) => command.run_command(ctx).await,
            Self::Info(command) => command.run_command(ctx).await
        }
    }
}

/// Open the database as writable in case we need to reload the hecks
async fn check_hecks_are_present(ctx: LuroFramework, guild_id: Option<Id<GuildMarker>>) -> anyhow::Result<()> {
    match guild_id {
        Some(guild_id) => {
            let guild_data = ctx.database.get_guild(&guild_id).await?;

            if guild_data.available_random_nsfw_hecks.is_empty() {
                ctx.database.reload_guild_heck_ids(&guild_id, true).await?;
            }

            if guild_data.available_random_sfw_hecks.is_empty() {
                ctx.database.reload_guild_heck_ids(&guild_id, false).await?;
            }
        }
        None => {
            let nsfw_hecks = ctx.database.get_hecks(true).await?;
            let sfw_hecks = ctx.database.get_hecks(false).await?;

            if nsfw_hecks.is_empty() {
                ctx.database.reload_global_heck_ids(true).await?;
            }

            if sfw_hecks.is_empty() {
                ctx.database.reload_global_heck_ids(false).await?;
            }
        }
    };

    debug!("hecks checked for being present, now returning");
    Ok(())
}

/// Open the database as writeable and remove a NSFW heck from it, returning the heck removed
async fn get_heck(
    ctx: &LuroFramework,
    id: Option<i64>,
    guild_id: Option<Id<GuildMarker>>,
    global: bool,
    nsfw: bool
) -> anyhow::Result<(Heck, usize)> {
    // Check to make sure our hecks are present, if not reload them
    // NOTE: This sets guild_id to false if we don't need to check for global hecks
    let heck_id;
    check_hecks_are_present(ctx.clone(), guild_id).await?;

    // A heck type to remove if we can't find it
    let no_heck = (
        Heck {
            heck_message: "No hecks found!".to_string(),
            author_id: PRIMARY_BOT_OWNER
        },
        69
    );

    if !global {
        let guild_id =
            guild_id.ok_or_else(|| Error::msg("Guild ID is not present. You can only use this option in a guild."))?;

        let guild_settings = ctx.database.get_guild(&guild_id).await?;

        heck_id = match id {
            Some(id) => usize::try_from(id)?,
            None => rand::thread_rng().gen_range(
                0..if nsfw {
                    let len = guild_settings.nsfw_hecks.len();
                    if len == 0 {
                        return Ok(no_heck);
                    }
                    len
                } else {
                    let len = guild_settings.sfw_hecks.len();
                    if len == 0 {
                        return Ok(no_heck);
                    }
                    len
                }
            )
        };

        let heck = ctx.database.get_heck(&heck_id, nsfw).await;

        Ok(match heck {
            Ok(heck) => (heck, heck_id),
            Err(_) => (
                Heck {
                    heck_message: "No hecks found!".to_string(),
                    author_id: PRIMARY_BOT_OWNER
                },
                69
            )
        })
    } else {
        debug!("user wants a global heck");
        // Use our specified ID if it is present, otherwise generate a random ID
        let _nsfw_hecks = ctx.database.get_hecks(true).await?;
        let _sfw_hecks = ctx.database.get_hecks(false).await?;
        // Try to use the id specified by the user, otherwise generate a random ID
        let heck_id = match id {
            Some(id) => id.try_into()?,
            None => rand::thread_rng().gen_range(
                0..if nsfw {
                    let len = ctx.database.available_random_nsfw_hecks.read().unwrap().len();
                    if len == 0 {
                        return Ok(no_heck);
                    }
                    len
                } else {
                    let len = ctx.database.available_random_sfw_hecks.read().unwrap().len();
                    if len == 0 {
                        return Ok(no_heck);
                    }
                    len
                }
            )
        };

        let heck = ctx.database.get_heck(&heck_id, nsfw).await;

        Ok(match heck {
            Ok(heck) => (heck, heck_id),
            Err(_) => (
                Heck {
                    heck_message: "No hecks found!".to_string(),
                    author_id: PRIMARY_BOT_OWNER
                },
                69
            )
        })
    }
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
