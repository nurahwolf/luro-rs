use anyhow::Error;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};

use twilight_model::application::{command::Command, interaction::Interaction};

use crate::functions::get_interaction_data;
use crate::models::luro::Luro;

use self::ban::BanCommand;
use self::kick::KickCommand;
use crate::commands::moderator::ban::ban;
use crate::commands::moderator::kick::kick;

mod ban;
mod kick;

pub fn commands() -> Vec<Command> {
    vec![ModeratorCommands::create_command().into()]
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "mod", desc = "Moderator Commands", dm_permission = false)]
enum ModeratorCommands {
    #[command(name = "ban")]
    Ban(BanCommand),
    #[command(name = "kick")]
    Kick(KickCommand),
}

pub async fn moderator(luro: &Luro, interaction: &Interaction) -> Result<(), Error> {
    let data = ModeratorCommands::from_interaction(CommandInputData::from(
        *get_interaction_data(interaction).await?,
    ))?;

    match data {
        ModeratorCommands::Ban(data) => ban(luro, interaction, data).await,
        ModeratorCommands::Kick(data) => kick(luro, interaction, data).await,
    }?;

    Ok(())
}
