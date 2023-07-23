use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{LuroContext, SlashResponse};

use self::{ban::BanCommand, kick::KickCommand, purge::PurgeCommand, settings::GuildSettingsCommand};
use super::LuroCommand;

mod ban;
mod kick;
mod purge;
mod settings;
mod assign;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "mod", desc = "Commands that can be used by moderators", dm_permission = false)]
pub enum ModeratorCommands {
    #[command(name = "ban")]
    Ban(BanCommand),
    #[command(name = "kick")]
    Kick(KickCommand),
    #[command(name = "purge")]
    Purge(PurgeCommand),
    #[command(name = "settings")]
    Setting(GuildSettingsCommand)
}

#[async_trait]
impl LuroCommand for ModeratorCommands {
    async fn run_commands(self, interaction: Interaction, ctx: LuroContext, shard: MessageSender) -> SlashResponse {
        // Call the appropriate subcommand.
        match self {
            Self::Ban(command) => command.run_command(interaction, ctx, shard).await,
            Self::Kick(command) => command.run_command(interaction, ctx, shard).await,
            Self::Purge(command) => command.run_command(interaction, ctx, shard).await,
            Self::Setting(command) => command.run_command(interaction, ctx, shard).await
        }
    }
}
