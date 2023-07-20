use async_trait::async_trait;
use twilight_gateway::MessageSender;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::application::{command::Command, interaction::Interaction};

use self::pause::PauseCommand;
use self::seek::SeekCommand;
use self::stop::StopCommand;
use self::volume::VolumeCommand;
use self::{join::JoinCommand, leave::LeaveCommand, play::PlayCommand};

use crate::{LuroContext, SlashResponse};

use super::LuroCommand;
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
    Stop(StopCommand)
}

#[async_trait]
impl LuroCommand for MusicCommands {
    async fn run_commands(self, interaction: Interaction, ctx: LuroContext, shard: MessageSender) -> SlashResponse {
        // Call the appropriate subcommand.
        match self {
            Self::Play(command) => command.run_command(interaction, ctx, shard).await,
            Self::Join(command) => command.run_command(interaction, ctx, shard).await,
            Self::Leave(command) => command.run_command(interaction, ctx, shard).await,
            Self::Pause(command) => command.run_command(interaction, ctx, shard).await,
            Self::Seek(command) => command.run_command(interaction, ctx, shard).await,
            Self::Volume(command) => command.run_command(interaction, ctx, shard).await,
            Self::Stop(command) => command.run_command(interaction, ctx, shard).await
        }
    }
}
