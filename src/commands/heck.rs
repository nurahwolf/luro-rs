use std::convert::TryInto;

use anyhow::Error;

use async_trait::async_trait;
use rand::Rng;

use twilight_interactions::command::{CommandModel, CreateCommand};

use tracing::{debug, trace};
use twilight_model::{
    application::command::Command,
    id::{marker::GuildMarker, Id},
    user::User
};

use crate::{
    models::Heck,
    responses::LuroSlash,
    LuroContext
};

use super::LuroCommand;

use self::{add::HeckAddCommand, info::HeckInfo, someone::HeckSomeoneCommand};

pub mod add;
mod info;
mod someone;

pub fn commands() -> Vec<Command> {
    vec![HeckCommands::create_command().into()]
}

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
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Add(command) => command.run_command(ctx).await,
            Self::Someone(command) => command.run_command(ctx).await,
            Self::Info(command) => command.run_command(ctx).await
        }
    }
}

/// Open the database as writable in case we need to reload the hecks
async fn check_hecks_are_present(ctx: LuroContext, guild_id: Option<Id<GuildMarker>>) -> anyhow::Result<()> {
    debug!("checking to make sure hecks are present");
    let (are_sfw_hecks_empty, are_nsfw_hecks_empty);

    match guild_id {
        Some(guild_id) => {
            trace!("checking if guild hecks are present");
            {
                let guild_db = ctx.guild_data.read();
                let guild_data = guild_db
                    .get(&guild_id)
                    .ok_or_else(|| Error::msg("There is no guild data available! Are you sure there are guild hecks here?"))?;
                are_sfw_hecks_empty = guild_data.hecks.sfw_heck_ids.is_empty();
                are_nsfw_hecks_empty = guild_data.hecks.nsfw_heck_ids.is_empty();
            }

            if are_sfw_hecks_empty || are_nsfw_hecks_empty {
                debug!("some hecks are empty, so we are reloading them");
                let mut guild_db = ctx.guild_data.write();
                let guild = guild_db.entry(guild_id);
                guild.and_modify(|guild| {
                    if are_sfw_hecks_empty {
                        guild.hecks.reload_sfw_heck_ids()
                    };

                    if are_nsfw_hecks_empty {
                        guild.hecks.reload_nsfw_heck_ids()
                    };
                });
            }
        }
        None => {
            trace!("checking if global hecks are present");
            let mut global_data = ctx.global_data.upgradable_read();
            are_sfw_hecks_empty = global_data.hecks.sfw_heck_ids.is_empty();
            are_nsfw_hecks_empty = global_data.hecks.nsfw_heck_ids.is_empty();

            if are_sfw_hecks_empty || are_nsfw_hecks_empty {
                debug!("some hecks are empty, so we are reloading them");
                global_data.with_upgraded(|data| {
                    if are_sfw_hecks_empty {
                        data.hecks.reload_sfw_heck_ids()
                    };

                    if are_nsfw_hecks_empty {
                        data.hecks.reload_nsfw_heck_ids()
                    };
                });
            }
        }
    };

    debug!("hecks checked for being present, now returning");
    Ok(())
}

/// Open the database as writeable and remove a NSFW heck from it, returning the heck removed
async fn get_heck(
    ctx: &LuroContext,
    id: Option<i64>,
    guild_id: Option<Id<GuildMarker>>,
    global: bool,
    nsfw: bool
) -> anyhow::Result<(Heck, usize)> {
    // Check to make sure our hecks are present, if not reload them
    // NOTE: This sets guild_id to false if we don't need to check for global hecks
    let (heck, heck_id);
    check_hecks_are_present(ctx.clone(), guild_id).await?;

    // A heck type to remove if we can't find it
    let no_heck = (
        Heck {
            heck_message: "No hecks found!".to_string(),
            author_id: 97003404601094144
        },
        69
    );

    if !global {
        let new_guild_settings;
        let guild_id =
            guild_id.ok_or_else(|| Error::msg("Guild ID is not present. You can only use this option in a guild."))?;

        {
            let mut guild_db = ctx.guild_data.write();
            let guild_settings = guild_db
                .get_mut(&guild_id)
                .ok_or_else(|| Error::msg("There are no settings for this guild. Blame Nurah."))?;

            heck_id = match id {
                Some(id) => id.try_into()?,
                None => {
                    let id = rand::thread_rng().gen_range(
                        0..if nsfw {
                            let len = guild_settings.hecks.nsfw_heck_ids.len();
                            if len == 0 {
                                return Ok(no_heck);
                            }
                            len
                        } else {
                            let len = guild_settings.hecks.sfw_heck_ids.len();
                            if len == 0 {
                                return Ok(no_heck);
                            }
                            len
                        }
                    );

                    if nsfw {
                        guild_settings.hecks.nsfw_heck_ids.remove(id)
                    } else {
                        guild_settings.hecks.sfw_heck_ids.remove(id)
                    }
                }
            };

            heck = if nsfw {
                guild_settings.hecks.nsfw_hecks.get(heck_id).cloned()
            } else {
                guild_settings.hecks.sfw_hecks.get(heck_id).cloned()
            };

            new_guild_settings = guild_settings.clone()
        }

        debug!("Saving new heck to disk");
        new_guild_settings.flush_to_disk(&guild_id).await?;

        debug!("heck created, returning");
        Ok(match heck {
            Some(heck) => (heck, heck_id),
            None => no_heck
        })
    } else {
        debug!("user wants a global heck");
        // Use our specified ID if it is present, otherwise generate a random ID
        let mut global_data = ctx.global_data.write();
        // Try to use the id specified by the user, otherwise generate a random ID
        let heck_id = match id {
            Some(id) => id.try_into()?,
            None => {
                let id = rand::thread_rng().gen_range(
                    0..if nsfw {
                        let len = global_data.hecks.nsfw_heck_ids.len();
                        if len == 0 {
                            return Ok(no_heck);
                        }
                        len
                    } else {
                        let len = global_data.hecks.sfw_heck_ids.len();
                        if len == 0 {
                            return Ok(no_heck);
                        }
                        len
                    }
                );

                if nsfw {
                    global_data.hecks.nsfw_heck_ids.remove(id)
                } else {
                    global_data.hecks.sfw_heck_ids.remove(id)
                }
            }
        };

        let heck = if nsfw {
            global_data.hecks.nsfw_hecks.get(heck_id)
        } else {
            global_data.hecks.sfw_hecks.get(heck_id)
        };

        Ok(match heck {
            Some(heck) => (heck.clone(), heck_id),
            None => (
                Heck {
                    heck_message: "No hecks found!".to_string(),
                    author_id: 97003404601094144
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
