use anyhow::Error;
use rand::Rng;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};

use twilight_model::{
    application::{command::Command, interaction::Interaction},
    user::User,
};

use crate::{
    data::{hecks::Heck, LuroData},
    functions::get_interaction_data,
    luro::Luro,
};

use self::user::HeckUserCommand;
use crate::commands::heck::user::user;

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
enum HeckCommands {
    #[command(name = "user")]
    User(HeckUserCommand),
}

pub async fn heck(luro: &Luro, interaction: &Interaction) -> Result<(), Error> {
    let data = HeckCommands::from_interaction(CommandInputData::from(
        *get_interaction_data(interaction).await?,
    ))?;

    match data {
        HeckCommands::User(data) => user(luro, interaction, data).await,
    }?;

    Ok(())
}

/// Open the database as writable in case we need to reload the hecks
async fn check_hecks_are_present(data: &LuroData) {
    let (are_sfw_hecks_empty, are_nsfw_hecks_empty);

    {
        let hecks = data.hecks.read().await.clone();
        are_sfw_hecks_empty = hecks.sfw_heck_ids.is_empty();
        are_nsfw_hecks_empty = hecks.nsfw_heck_ids.is_empty();
    }

    if are_sfw_hecks_empty {
        let hecks = &mut data.hecks.write().await;
        hecks.reload_sfw_heck_ids()
    };

    if are_nsfw_hecks_empty {
        let hecks = &mut data.hecks.write().await;
        hecks.reload_nsfw_heck_ids()
    };
}

/// Open the database as writeable and remove a NSFW heck from it, returning the heck removed
async fn get_heck(data: &LuroData, heck_id: Option<usize>, nsfw: bool) -> (Heck, usize) {
    let hecks;

    // Check to make sure our hecks are present, if not reload them
    check_hecks_are_present(data).await;

    // Create a copy of the hecks, so that we can do some operations
    {
        hecks = data.hecks.read().await.clone();
    }

    // Use our specified ID if it is present, otherwise generate a random ID
    let heck_id = match heck_id {
        Some(id_specified) => id_specified,
        None => rand::thread_rng().gen_range(
            0..if nsfw {
                hecks.nsfw_heck_ids.len()
            } else {
                hecks.sfw_heck_ids.len()
            },
        ),
    };

    // Attempt to get our heck, otherwise return an error
    let heck = if nsfw {
        hecks.nsfw_hecks.get(heck_id)
    } else {
        hecks.sfw_hecks.get(heck_id)
    };

    // Remove the used heck ID. NOTE, we don't know if our heck is valid, and this is a good way to remove an invalid heck ID in case it is not present.
    {
        let hecks = &mut data.hecks.write().await;
        if nsfw {
            hecks.nsfw_heck_ids.remove(heck_id)
        } else {
            hecks.sfw_heck_ids.remove(heck_id)
        };
    }

    // Validate our heck
    match heck {
        Some(heck) => (heck.clone(), heck_id),
        None => (
            Heck {
                heck_message: "Heck not found!".to_string(),
                author_id: 97003404601094144,
            },
            69,
        ),
    }
}

/// Replace <user> and <author> with the hecked user's username and author's name
async fn format_heck(heck: &Heck, heck_author: &User, hecked_user: &User) -> Heck {
    Heck {
        heck_message: heck
            .heck_message
            .replace("<user>", &format!("<@{}>", &hecked_user.id))
            .replace("<author>", &format!("<@{}>", &heck_author.id)),
        author_id: heck.author_id,
    }
}
