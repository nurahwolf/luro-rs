use anyhow::{Context, Error};
use twilight_gateway::MessageSender;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::{command::Command, interaction::Interaction};

use self::pause::PauseCommand;
use self::seek::SeekCommand;
use self::stop::StopCommand;
use self::volume::VolumeCommand;
use self::{join::JoinCommand, leave::LeaveCommand, play::PlayCommand};

use crate::commands::music::join::join;
use crate::commands::music::leave::leave;
use crate::commands::music::pause::pause;
use crate::commands::music::play::play;
use crate::commands::music::seek::seek;
use crate::commands::music::stop::stop;
use crate::commands::music::volume::volume;
use crate::framework::LuroFramework;
use crate::interactions::InteractionResponse;

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
pub enum MusicCommands {
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

impl MusicCommands {
    pub async fn run(
        interaction: &Interaction,
        ctx: &LuroFramework,
        data: CommandData,
        shard: MessageSender,
    ) -> Result<InteractionResponse, Error> {
        // Parse the command data into a structure using twilight-interactions.
        let command =
            MusicCommands::from_interaction(data.into()).context("failed to parse command data")?;

        match command {
            MusicCommands::Play(data) => play(ctx, interaction, data).await,
            MusicCommands::Join(data) => join(interaction, shard, data).await,
            MusicCommands::Leave(_) => leave(ctx, interaction, shard).await,
            MusicCommands::Pause(_) => pause(ctx, interaction).await,
            MusicCommands::Seek(data) => seek(ctx, interaction, data).await,
            MusicCommands::Volume(data) => volume(ctx, interaction, data).await,
            MusicCommands::Stop(_) => stop(ctx, interaction).await,
        }
    }
}
