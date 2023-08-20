use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::id::Id;

use crate::interaction::LuroSlash;

use crate::luro_command::LuroCommand;

use self::abuse::AbuseCommand;
use self::assign::AssignCommand;
use self::clear_warnings::OwnerClearWarning;
use self::commands::OwnerCommandsCommand;
use self::config::ConfigCommand;
use self::fakeban::FakeBan;
use self::flush::Flush;
use self::get_message::OwnerGetMessage;
use self::guilds::OwnerGuildsCommand;
use self::load_users::OwnerLoadUsers;
use self::log::LogCommand;
use self::modify::Modify;
use self::modify_role::ModifyRoleCommand;

mod abuse;
mod assign;
mod clear_warnings;
mod commands;
mod config;
mod flush;
mod get_message;
mod guilds;
mod load_users;
mod log;
mod modify;
mod modify_role;
mod fakeban;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges uwu!")]
pub enum OwnerCommands {
    #[command(name = "flush")]
    Flush(Flush),
    #[command(name = "log")]
    Log(LogCommand),
    #[command(name = "assign")]
    Assign(Box<AssignCommand>),
    #[command(name = "modify_role")]
    ModifyRole(ModifyRoleCommand),
    #[command(name = "commands")]
    Commands(OwnerCommandsCommand),
    #[command(name = "abuse")]
    Abuse(Box<AbuseCommand>),
    #[command(name = "load_users")]
    LoadUsers(OwnerLoadUsers),
    #[command(name = "clear_warnings")]
    ClearWarning(Box<OwnerClearWarning>),
    #[command(name = "guilds")]
    Guilds(OwnerGuildsCommand),
    #[command(name = "get_message")]
    GetMessage(Box<OwnerGetMessage>),
    #[command(name = "config")]
    Config(ConfigCommand),
    #[command(name = "modify")]
    Modify(Modify),
    #[command(name = "fakeban")]
    FakeBan(FakeBan)
}

impl LuroCommand for OwnerCommands {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction_author = ctx.interaction.author().unwrap();
        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live

        for (id, _) in ctx.framework.database.get_staff().await? {
            if interaction_author.id == id {
                owner_match = true
            }
        }

        // If we don't have a match, bitch at the user
        if !owner_match {
            return ctx
                .not_owner_response(
                    &interaction_author.id,
                    &ctx.interaction.guild_id,
                    match self {
                        Self::Abuse(_) => "owner_abuse",
                        Self::Assign(_) => "owner_assign",
                        Self::ClearWarning(_) => "owner_clearwarning",
                        Self::Commands(_) => "owner_commands",
                        Self::Config(_) => "owner_config",
                        Self::GetMessage(_) => "owner_getmessage",
                        Self::Guilds(_) => "owner_guilds",
                        Self::LoadUsers(_) => "owner_loadusers",
                        Self::Log(_) => "owner_log",
                        Self::ModifyRole(_) => "owner_modify",
                        Self::Flush(_) => "owner_save",
                        Self::Modify(_) => "owner_modify",
                        Self::FakeBan(_) => "owner_fakeban"
                    }
                )
                .await;
        }

        // We know the user is good, so call the appropriate subcommand.
        match self {
            Self::Abuse(command) => command.run_command(ctx).await,
            Self::Assign(command) => command.run_command(ctx).await,
            Self::ClearWarning(command) => command.run_command(ctx).await,
            Self::Commands(command) => command.run_command(ctx).await,
            Self::Config(command) => command.run_command(ctx).await,
            Self::GetMessage(command) => command.run_command(ctx).await,
            Self::Guilds(command) => command.run_command(ctx).await,
            Self::LoadUsers(command) => command.run_command(ctx).await,
            Self::Log(command) => command.run_command(ctx).await,
            Self::ModifyRole(command) => command.run_command(ctx).await,
            Self::Flush(command) => command.run_command(ctx).await,
            Self::Modify(command) => command.run_command(ctx).await,
            Self::FakeBan(command) => command.run_command(ctx).await

        }
    }

    async fn handle_model(data: ModalInteractionData, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut message_id = None;
        let mut channel_id = None;

        for row in &data.components {
            for component in &row.components {
                if let Some(ref value) = component.value && component.custom_id.as_str() == "message-id" {
                    message_id = Some(value.clone());
                }
                if let Some(ref value) = component.value && component.custom_id.as_str() == "channel-id" {
                    channel_id = Some(value.clone());
                }
            }
        }

        let message_id = match message_id {
            Some(message_id) => Id::new(message_id.parse()?),
            None => return ctx.respond(|r| r.content("No message ID!").ephemeral()).await
        };

        let channel_id = match channel_id {
            Some(channel_id) => Id::new(channel_id.parse()?),
            None => return ctx.respond(|r| r.content("No channel ID!").ephemeral()).await
        };

        let message = ctx
            .framework
            .twilight_client
            .message(channel_id, message_id)
            .await?
            .model()
            .await?;
        let mut embed = message.embeds.first().unwrap().clone();

        for row in &data.components {
            for component in &row.components {
                if let Some(ref value) = component.value && component.custom_id.as_str() == "embed-title" {
                    embed.title = Some(value.clone());
                }
                if let Some(ref value) = component.value && component.custom_id.as_str() == "embed-description" {
                    embed.description = Some(value.clone());
                }
            }
        }

        ctx.framework
            .twilight_client
            .update_message(channel_id, message_id)
            .embeds(Some(vec![embed]).as_deref())
            .await?;

        ctx.respond(|r| r.content("All done!").ephemeral()).await
    }
}
