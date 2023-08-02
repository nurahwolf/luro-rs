use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::LuroSlash;

use self::{
    ban::BanCommand, kick::KickCommand, purge::PurgeCommand, settings::GuildSettingsCommand, warn::ModeratorWarnCommand
};
use crate::traits::luro_command::LuroCommand;

mod assign;
mod ban;
mod kick;
mod purge;
mod settings;
pub mod warn;

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
    Setting(GuildSettingsCommand),
    #[command(name = "warn")]
    Warn(ModeratorWarnCommand)
}

#[async_trait]
impl LuroCommand for ModeratorCommands {
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Ban(command) => command.run_command(ctx).await,
            Self::Kick(command) => command.run_command(ctx).await,
            Self::Purge(command) => command.run_command(ctx).await,
            Self::Setting(command) => command.run_command(ctx).await,
            Self::Warn(command) => command.run_command(ctx).await
        }
    }
}
