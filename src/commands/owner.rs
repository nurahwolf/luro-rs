use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::responses::LuroSlash;
use crate::BOT_OWNER;

use super::LuroCommand;

use self::assign::AssignCommand;
use self::commands::OwnerCommandsCommand;
use self::log::LogCommand;
use self::modify_role::ModifyRoleCommand;
use self::save::SaveCommand;

mod assign;
mod commands;
mod log;
mod modify_role;
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
    Commands(OwnerCommandsCommand)
}

#[async_trait]
impl LuroCommand for OwnerCommands {
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction_author = ctx.author()?;

        if !interaction_author.id.get() == BOT_OWNER {
            return ctx.not_owner_response().await;
        }

        // Call the appropriate subcommand.
        match self {
            Self::Save(command) => command.run_command(ctx).await,
            Self::Log(command) => command.run_command(ctx).await,
            Self::Assign(command) => command.run_command(ctx).await,
            Self::Modify(command) => command.run_command(ctx).await,
            Self::Commands(command) => command.run_command(ctx).await
        }
    }
}
