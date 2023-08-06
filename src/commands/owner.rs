use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;

use self::abuse::AbuseCommand;
use self::assign::AssignCommand;
use self::clear_warnings::OwnerClearWarning;
use self::commands::OwnerCommandsCommand;
use self::get_message::OwnerGetMessage;
use self::guilds::OwnerGuildsCommand;
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
mod get_message;
mod guilds;
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
    #[command(name = "clear_warnings")]
    ClearWarning(Box<OwnerClearWarning>),
    #[command(name = "guilds")]
    Guilds(OwnerGuildsCommand),
    #[command(name = "get_message")]
    GetMessage(Box<OwnerGetMessage>)
}

#[async_trait]
impl LuroCommand for OwnerCommands {
    async fn run_commands(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live
        for owner in &ctx.data_global.read().owners {
            if slash.interaction.author_id().context("Expected interaction author ID")? == owner.id {
                owner_match = true
            }
        }

        // If we don't have a match, bitch at the user
        if !owner_match {
            return ctx
                .not_owner_response(
                    &slash.interaction.author_id().context("Expected interaction author ID")?,
                    &slash.interaction.guild_id.clone(),
                    match self {
                        Self::Save(_) => "owner_save",
                        Self::Log(_) => "owner_log",
                        Self::Assign(_) => "owner_assign",
                        Self::Modify(_) => "owner_modify",
                        Self::Commands(_) => "owner_commands",
                        Self::Reload(_) => "owner_reload",
                        Self::SaveGuilds(_) => "owner_saveguilds",
                        Self::Abuse(_) => "owner_abuse",
                        Self::LoadUsers(_) => "owner_loadusers",
                        Self::ClearWarning(_) => "owner_clearwarning",
                        Self::Guilds(_) => "owner_guilds",
                        Self::GetMessage(_) => "owner_getmessage"
                    },
                    &mut slash
                )
                .await;
        }

        // We know the user is good, so call the appropriate subcommand.
        match self {
            Self::Save(command) => command.run_command(ctx, slash).await,
            Self::Log(command) => command.run_command(ctx, slash).await,
            Self::Assign(command) => command.run_command(ctx, slash).await,
            Self::Modify(command) => command.run_command(ctx, slash).await,
            Self::Commands(command) => command.run_command(ctx, slash).await,
            Self::Reload(command) => command.run_command(ctx, slash).await,
            Self::SaveGuilds(command) => command.run_command(ctx, slash).await,
            Self::Abuse(command) => command.run_command(ctx, slash).await,
            Self::LoadUsers(command) => command.run_command(ctx, slash).await,
            Self::ClearWarning(command) => command.run_command(ctx, slash).await,
            Self::Guilds(command) => command.run_command(ctx, slash).await,
            Self::GetMessage(command) => command.run_command(ctx, slash).await
        }
    }
}
