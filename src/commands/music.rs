use anyhow::Error;
use twilight_gateway::stream::ShardRef;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};

use twilight_model::application::{command::Command, interaction::Interaction};

use crate::{functions::get_interaction_data, luro::Luro};

use self::pause::PauseCommand;
use self::seek::SeekCommand;
use self::stop::StopCommand;
use self::volume::VolumeCommand;
use self::{join::JoinCommand, leave::LeaveCommand, play::PlayCommand};

use super::create_response;
use crate::commands::music::join::join;
use crate::commands::music::leave::leave;
use crate::commands::music::pause::pause;
use crate::commands::music::play::play;
use crate::commands::music::seek::seek;
use crate::commands::music::stop::stop;
use crate::commands::music::volume::volume;

mod join;
mod leave;
mod pause;
mod play;
mod seek;
mod stop;
mod volume;

pub fn commands() -> Vec<Command> {
    vec![MusicCommands::create_command().into()]
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "music", desc = "Music commands!", dm_permission = false)]
enum MusicCommands {
    #[command(name = "play")]
    Play(PlayCommand),
    #[command(name = "join")]
    Join(JoinCommand),
    #[command(name = "leave")]
    Leave(LeaveCommand),
    #[command(name = "pause")]
    Pause(PauseCommand),
    #[command(name = "seek")]
    Seek(SeekCommand),
    #[command(name = "volume")]
    Volume(VolumeCommand),
    #[command(name = "stop")]
    Stop(StopCommand),
}

pub async fn music(
    luro: &Luro,
    interaction: &Interaction,
    shard: ShardRef<'_>,
) -> Result<(), Error> {
    let data = MusicCommands::from_interaction(CommandInputData::from(
        *get_interaction_data(interaction).await?,
    ))?;

    match data {
        MusicCommands::Play(data) => play(luro, interaction, data).await,
        MusicCommands::Join(data) => join(luro, interaction, shard, data).await,
        MusicCommands::Leave(_) => leave(luro, interaction, shard).await,
        MusicCommands::Pause(_) => pause(luro, interaction).await,
        MusicCommands::Seek(data) => seek(luro, interaction, data).await,
        MusicCommands::Volume(data) => volume(luro, interaction, data).await,
        MusicCommands::Stop(_) => stop(luro, interaction).await,
    }?;

    Ok(())
}
