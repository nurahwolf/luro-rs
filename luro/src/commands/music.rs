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

use crate::LuroContext;

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
    Stop(StopCommand)
}

impl MusicCommands {
    pub async fn run(
        interaction: &Interaction,
        ctx: &LuroContext,
        data: CommandData,
        shard: MessageSender
    ) -> Result<InteractionResponse, Error> {
        // Parse the command data into a structure using twilight-interactions.
        let command = Self::from_interaction(data.into()).context("failed to parse command data")?;

        match command {
            Self::Play(data) => data.run(interaction, ctx).await,
            Self::Join(data) => data.run(interaction, ctx, shard).await,
            Self::Leave(data) => data.run(interaction, ctx, shard).await,
            Self::Pause(data) => data.run(interaction, ctx).await,
            Self::Seek(data) => data.run(interaction, ctx).await,
            Self::Volume(data) => data.run(interaction, ctx).await,
            Self::Stop(data) => data.run(interaction, ctx).await
        }
    }
}
