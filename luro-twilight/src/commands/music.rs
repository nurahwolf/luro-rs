use luro_framework::{CommandInteraction, CreateLuroCommand, ExecuteLuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

use self::info::InfoCommand;
use self::pause::PauseCommand;
use self::seek::SeekCommand;
use self::stop::StopCommand;
use self::volume::VolumeCommand;
use self::{join::JoinCommand, leave::LeaveCommand, play::PlayCommand};

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
pub enum Music {
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
    Info(InfoCommand),
}

impl CreateLuroCommand for Music {}

impl ExecuteLuroCommand for Music {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        // Call the appropriate subcommand.
        match self {
            Self::Play(command) => command.interaction_command(ctx).await,
            Self::Join(command) => command.interaction_command(ctx).await,
            Self::Leave(command) => command.interaction_command(ctx).await,
            Self::Pause(command) => command.interaction_command(ctx).await,
            Self::Seek(command) => command.interaction_command(ctx).await,
            Self::Volume(command) => command.interaction_command(ctx).await,
            Self::Stop(command) => command.interaction_command(ctx).await,
            Self::Info(command) => command.interaction_command(ctx).await,
        }
    }
}
