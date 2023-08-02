use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;

use self::abuse::AbuseCommand;
use self::assign::AssignCommand;
use self::clear_warnings::OwnerClearWarning;
use self::commands::OwnerCommandsCommand;
use self::load_users::OwnerLoadUsers;
use self::log::LogCommand;
use self::modify_role::ModifyRoleCommand;
use self::reload::ReloadCommand;
use self::save::SaveCommand;
use self::save_guilds::SaveGuildsCommand;

mod abuse;
mod assign;
mod clear_warnings;
mod commands;
mod load_users;
mod log;
mod modify_role;
mod reload;
mod save;
mod save_guilds;

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
    Reload(ReloadCommand),
    #[command(name = "save_guilds")]
    SaveGuilds(SaveGuildsCommand),
    #[command(name = "abuse")]
    Abuse(Box<AbuseCommand>),
    #[command(name = "load_users")]
    LoadUsers(OwnerLoadUsers),
    #[command(name = "clear_warning")]
    ClearWarning(OwnerClearWarning)
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
            Self::Reload(command) => command.run_command(ctx).await,
            Self::SaveGuilds(command) => command.run_command(ctx).await,
            Self::Abuse(command) => command.run_command(ctx).await,
            Self::LoadUsers(command) => command.run_command(ctx).await,
            Self::ClearWarning(command) => command.run_command(ctx).await
        }
    }
}
