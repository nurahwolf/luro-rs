use std::convert::TryInto;

use anyhow::Context;
use parking_lot::RwLock;
use rand::Rng;
use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    application::{
        command::Command,
        interaction::{application_command::CommandData, Interaction},
    },
    user::User,
};

use crate::{framework::GlobalData, hecks::Heck, LuroContext, SlashResponse};

use self::{add::HeckAddCommand, info::HeckInfo, user::HeckUserCommand};

pub mod add;
mod info;
mod user;

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
    User(HeckUserCommand),
    #[command(name = "info")]
    Info(HeckInfo),
}

impl HeckCommands {
    pub async fn run(
        self,
        ctx: LuroContext,
        interaction: &Interaction,
        data: CommandData,
    ) -> SlashResponse {
        // Parse the command data into a structure using twilight-interactions.
        let command =
            HeckCommands::from_interaction(data.into()).context("failed to parse command data")?;

        // Call the appropriate subcommand.
        Ok(match command {
            Self::Add(command) => command.run(ctx, interaction).await?,
            Self::User(command) => command.run(ctx, interaction).await?,
            Self::Info(command) => command.run(ctx, interaction).await?,
        })
    }
}

/// Open the database as writable in case we need to reload the hecks
async fn check_hecks_are_present(global_data: &RwLock<GlobalData>) -> anyhow::Result<()> {
    let (are_sfw_hecks_empty, are_nsfw_hecks_empty);

    {
        // Check to see if the hecks are empty, if not then there is no need to open as write
        let global_data = global_data.read();
        are_sfw_hecks_empty = global_data.hecks.sfw_heck_ids.is_empty();
        are_nsfw_hecks_empty = global_data.hecks.nsfw_heck_ids.is_empty();
    }

    if are_sfw_hecks_empty || are_nsfw_hecks_empty {
        // Hecks are empty, so open as write and reload them
        let mut global_data = global_data.write();
        if are_sfw_hecks_empty {
            global_data.hecks.reload_sfw_heck_ids()
        };

        if are_nsfw_hecks_empty {
            global_data.hecks.reload_nsfw_heck_ids()
        };
    }

    Ok(())
}

/// Open the database as writeable and remove a NSFW heck from it, returning the heck removed
async fn get_heck(ctx: LuroContext, id: Option<i64>, nsfw: bool) -> anyhow::Result<(Heck, usize)> {
    // Check to make sure our hecks are present, if not reload them
    check_hecks_are_present(&ctx.global_data).await?;
    let heck_id;
    {
        // Use our specified ID if it is present, otherwise generate a random ID
        let global_data = ctx.global_data.read();
        heck_id = match id {
            Some(id) => id.try_into()?,
            None => rand::thread_rng().gen_range(
                0..if nsfw {
                    global_data.hecks.nsfw_heck_ids.len()
                } else {
                    global_data.hecks.sfw_heck_ids.len()
                },
            ),
        };
    }

    // Remove the used heck ID. NOTE, we don't know if our heck is valid, and this is a good way to remove an invalid heck ID in case it is not present.
    let mut global_data = ctx.global_data.write();
    // Attempt to get our heck, otherwise return an error

    if nsfw {
        global_data.hecks.nsfw_heck_ids.remove(heck_id)
    } else {
        global_data.hecks.sfw_heck_ids.remove(heck_id)
    };

    let heck = if nsfw {
        global_data.hecks.nsfw_hecks.get(heck_id)
    } else {
        global_data.hecks.sfw_hecks.get(heck_id)
    };

    // Validate our heck
    Ok(match heck {
        Some(heck) => (heck.clone(), heck_id),
        None => (
            Heck {
                heck_message: "Heck not found!".to_string(),
                author_id: 97003404601094144,
            },
            69,
        ),
    })
}

/// Replace <user> with <@hecked_user> and <author> with the caller of the heck command
async fn format_heck(heck: &Heck, heck_author: &User, hecked_user: &User) -> Heck {
    Heck {
        heck_message: heck
            .heck_message
            .replace("<user>", &format!("<@{}>", &hecked_user.id))
            .replace("<author>", &format!("<@{}>", &heck_author.id)),
        author_id: heck.author_id,
    }
}
