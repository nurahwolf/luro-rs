use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::responses::LuroSlash;

use super::LuroCommand;

use self::assign::AssignCommand;
use self::commands::OwnerCommandsCommand;
use self::log::LogCommand;
use self::modify_role::ModifyRoleCommand;
use self::reload::ReloadCommand;
use self::save::SaveCommand;

mod assign;
mod commands;
mod log;
mod modify_role;
mod reload;
mod save;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges uwu!")]
pub enum OwnerCommands {
    #[command(name = "save")]
    Save(SaveCommand),
    #[command(name = "log")]
    Log(LogCommand),
    #[command(name = "assign")]
    Assign(Box<AssignCommand>),
    #[command(name = "modify_role")]
    Modify(ModifyRoleCommand),
    #[command(name = "commands")]
    Commands(OwnerCommandsCommand),
    #[command(name = "reload")]
    Reload(ReloadCommand)
}

#[async_trait]
impl LuroCommand for OwnerCommands {
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction_author = ctx.author()?;
        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live
        for owner in &ctx.luro.global_data.read().owners {
            if interaction_author.id == owner.id {
                owner_match = true
            }
        }

        // If we don't have a match, bitch at the user
        if !owner_match {
            return ctx.not_owner_response().await;
        }

        // We know the user is good, so call the appropriate subcommand.
        match self {
            Self::Save(command) => command.run_command(ctx).await,
            Self::Log(command) => command.run_command(ctx).await,
            Self::Assign(command) => command.run_command(ctx).await,
            Self::Modify(command) => command.run_command(ctx).await,
            Self::Commands(command) => command.run_command(ctx).await,
            Self::Reload(command) => command.run_command(ctx).await
        }
    }
}
