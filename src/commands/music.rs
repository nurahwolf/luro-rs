use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use self::info::InfoCommand;
use self::pause::PauseCommand;
use self::seek::SeekCommand;
use self::stop::StopCommand;
use self::volume::VolumeCommand;
use self::{join::JoinCommand, leave::LeaveCommand, play::PlayCommand};

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
mod info;
mod join;
mod leave;
mod pause;
mod play;
mod seek;
mod stop;
mod volume;

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
    #[command(name = "info")]
    Info(InfoCommand)
}

#[async_trait]
impl LuroCommand for MusicCommands {
    async fn run_commands(self, ctx: &LuroContext, slash: LuroResponse) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Play(command) => command.run_command(ctx, slash).await,
            Self::Join(command) => command.run_command(ctx, slash).await,
            Self::Leave(command) => command.run_command(ctx, slash).await,
            Self::Pause(command) => command.run_command(ctx, slash).await,
            Self::Seek(command) => command.run_command(ctx, slash).await,
            Self::Volume(command) => command.run_command(ctx, slash).await,
            Self::Stop(command) => command.run_command(ctx, slash).await,
            Self::Info(command) => command.run_command(ctx, slash).await
        }
    }
}
