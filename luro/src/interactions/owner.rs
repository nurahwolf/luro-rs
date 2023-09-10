use luro_model::BOT_OWNERS;
use std::fmt::Write;
use tracing::warn;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::application::interaction::InteractionData;
use twilight_model::channel::message::component::SelectMenuType;
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::Id;

use crate::interaction::LuroSlash;
use luro_model::database_driver::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;

mod abuse;
mod assign;
mod clear_warnings;
mod commands;
mod config;
mod fakeban;
mod flush;
mod get_message;
mod guilds;
mod load_users;
mod log;
pub mod mass_assign;
mod modify;
mod modify_role;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges!")]
pub enum Owner {
    #[command(name = "abuse")]
    Abuse(abuse::AbuseCommand),
    #[command(name = "assign")]
    Assign(assign::AssignCommand),
    #[command(name = "clear_warnings")]
    ClearWarning(clear_warnings::OwnerClearWarning),
    #[command(name = "commands")]
    Commands(commands::OwnerCommandsCommand),
    #[command(name = "fakeban")]
    FakeBan(fakeban::FakeBan),
    #[command(name = "log")]
    Log(log::LogCommand),
    #[command(name = "mass_assign")]
    MassAssign(mass_assign::MassAssign),
    #[command(name = "modify")]
    Modify(modify::Modify),
    #[command(name = "modify_role")]
    ModifyRole(modify_role::ModifyRoleCommand),
    #[command(name = "flush")]
    Flush(flush::Flush),
}

// pub enum OwnerCommands {
//     #[command(name = "config")]
//     #[command(name = "get_message")]
//     #[command(name = "guilds")]
//     #[command(name = "load_users")]
//     Config(ConfigCommand),
//     GetMessage(OwnerGetMessage),
//     Guilds(OwnerGuildsCommand),
//     LoadUsers(OwnerLoadUsers),
// }

impl LuroCommand for Owner {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let interaction_author = ctx.interaction.author().unwrap();

        let staff = match ctx.framework.database.get_staff().await {
            Ok(data) => data.keys().copied().collect(),
            Err(why) => {
                warn!(why = ?why, "Failed to load staff from database, falling back to hardcoded staff members");
                BOT_OWNERS.to_vec()
            }
        };

        // If we don't have a match, bitch at the user
        if !staff.contains(&interaction_author.id) {
            return ctx
                .not_owner_response(
                    &interaction_author.id,
                    &ctx.interaction.guild_id,
                    match self {
                        Self::Abuse(_) => "owner_abuse",
                        Self::Assign(_) => "owner_assign",
                        Self::ClearWarning(_) => "owner_clearwarning",
                        Self::Commands(_) => "owner_commands",
                        // Self::Config(_) => "owner_config",
                        // Self::GetMessage(_) => "owner_getmessage",
                        // Self::Guilds(_) => "owner_guilds",
                        // Self::LoadUsers(_) => "owner_loadusers",
                        Self::Log(_) => "owner_log",
                        Self::ModifyRole(_) => "owner_modify",
                        Self::Flush(_) => "owner_save",
                        Self::Modify(_) => "owner_modify",
                        Self::FakeBan(_) => "owner_fakeban",
                        Self::MassAssign(_) => "mass_assign",
                    },
                )
                .await;
        }

        // We know the user is good, so call the appropriate subcommand.
        match self {
            Self::Abuse(command) => command.run_command(ctx).await,
            Self::Assign(command) => command.run_command(ctx).await,
            Self::ClearWarning(command) => command.run_command(ctx).await,
            Self::Commands(command) => command.run_command(ctx).await,
            // Self::Config(command) => command.run_command(ctx).await,
            Self::FakeBan(command) => command.run_command(ctx).await,
            Self::Flush(command) => command.run_command(ctx).await,
            // Self::GetMessage(command) => command.run_command(ctx).await,
            // Self::Guilds(command) => command.run_command(ctx).await,
            // Self::LoadUsers(command) => command.run_command(ctx).await,
            Self::Log(command) => command.run_command(ctx).await,
            Self::MassAssign(command) => command.run_command(ctx).await,
            Self::Modify(command) => command.run_command(ctx).await,
            Self::ModifyRole(command) => command.run_command(ctx).await,
        }
    }

    async fn handle_model<D: LuroDatabaseDriver>(data: ModalInteractionData, ctx: LuroSlash<D>) -> anyhow::Result<()> {
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
            None => return ctx.respond(|r| r.content("No message ID!").ephemeral()).await,
        };

        let channel_id = match channel_id {
            Some(channel_id) => Id::new(channel_id.parse()?),
            None => return ctx.respond(|r| r.content("No channel ID!").ephemeral()).await,
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

    async fn handle_component<D: LuroDatabaseDriver>(
        self,
        data: Box<MessageComponentInteractionData>,
        ctx: LuroSlash<D>,
    ) -> anyhow::Result<()> {
        match data.custom_id.as_str() {
            "mass-assign-selector" => component_selector(ctx).await,
            "mass-assign-roles" | "mass-assign-remove" => component_roles(data, ctx).await,
            _ => ctx.respond(|r| r.content("Unknown Command!").ephemeral()).await,
        }
    }
}

async fn component_selector<D: LuroDatabaseDriver>(ctx: LuroSlash<D>) -> anyhow::Result<()> {
    let mut roles_string = String::new();
    let guild_id = ctx.interaction.guild_id.unwrap();
    let mut data = None;
    let mut interaction = ctx.interaction.clone();

    while data.is_none() {
        match interaction.data {
            Some(InteractionData::MessageComponent(new_data)) => {
                data = Some(new_data);
                break;
            }
            _ => {
                if let Some(message) = interaction.message {
                    interaction = ctx.framework.database.get_interaction(&message.id.to_string()).await?;
                }
            }
        }
    }

    let data = data.unwrap();

    let mut roles: Vec<Id<RoleMarker>> = data.values.iter().map(|role| Id::new(role.parse::<u64>().unwrap())).collect();

    // let guild = ctx.framework.twilight_cache.guild_members(guild_id).unwrap();
    let guild = ctx
        .framework
        .twilight_client
        .guild_members(guild_id)
        .limit(1000)
        .await?
        .model()
        .await?;
    let mut users = vec![];
    for member in guild.into_iter() {
        if let Ok(user) = ctx
            .framework
            .database
            .get_user(&member.user.id)
            .await
        {
            users.push(user)
        }
    }

    match roles.is_empty() {
        true => roles.push(guild_id.cast()),
        false => users.retain(|user| {
            let mut found = false;
            match user.guilds.get(&guild_id) {
                Some(guild_data) => {
                    for role in &roles {
                        if guild_data.role_ids.contains(role) {
                            found = true
                        }
                    }
                }
                None => found = false,
            };
            found
        }),
    };

    for role in &roles {
        writeln!(roles_string, "- <@&{role}>")?;
    }

    ctx.respond(|response| {
        {
            response
                .content(format!("Found `{}` users with the role(s):\n{roles_string}\nFirst Menu: The roles to apply\nSecond Menu: The roles to remove", users.len()))
                .components(|components| {
                    components
                        .action_row(|row| {
                            row.component(|component| {
                                component.select_menu(|menu| {
                                    menu.custom_id("mass-assign-roles")
                                        .kind(SelectMenuType::Role)
                                        .max_values(25)
                                        .min_values(1)
                                })
                            })
                        })
                        .action_row(|row| {
                            row.component(|component| {
                                component.select_menu(|menu| {
                                    menu.custom_id("mass-assign-remove")
                                        .kind(SelectMenuType::Role)
                                        .max_values(25)
                                        .min_values(1)
                                })
                            })
                        })
                })
        }
        .ephemeral()
    })
    .await
}

async fn component_roles<D: LuroDatabaseDriver>(
    data: Box<MessageComponentInteractionData>,
    ctx: LuroSlash<D>,
) -> anyhow::Result<()> {
    let guild_id = ctx.interaction.guild_id.unwrap();

    let mut roles: Vec<Id<RoleMarker>> = data.values.iter().map(|role| Id::new(role.parse::<u64>().unwrap())).collect();

    // let guild = ctx.framework.twilight_cache.guild_members(guild_id).unwrap();
    let guild = ctx
        .framework
        .twilight_client
        .guild_members(guild_id)
        .limit(1000)
        .await?
        .model()
        .await?;
    let mut users = vec![];
    for member in guild.into_iter() {
        if let Ok(user) = ctx
            .framework
            .database
            .get_user(&member.user.id)
            .await
        {
            users.push(user)
        }
    }

    match roles.is_empty() {
        true => roles.push(guild_id.cast()),
        false => users.retain(|user| {
            let mut found = false;
            match user.guilds.get(&guild_id) {
                Some(guild_data) => {
                    for role in &roles {
                        if guild_data.role_ids.contains(role) {
                            found = true
                        }
                    }
                }
                None => found = false,
            };
            found
        }),
    };

    let mut actions_performed = 0;
    let mut errors = 0;
    match data.custom_id.as_str() {
        "mass-assign-roles" => {
            for user in users {
                for role in &roles {
                    match ctx
                        .framework
                        .twilight_client
                        .add_guild_member_role(guild_id, user.id, *role)
                        .await
                    {
                        Ok(_) => actions_performed += 1,
                        Err(_) => errors += 1,
                    };
                }
            }
        }
        "mass-assign-remove" => {
            for user in users {
                for role in &roles {
                    match ctx
                        .framework
                        .twilight_client
                        .remove_guild_member_role(guild_id, user.id, *role)
                        .await
                    {
                        Ok(_) => actions_performed += 1,
                        Err(_) => errors += 1,
                    };
                }
            }
        }
        _ => return ctx.respond(|r| r.content("It's fucked").ephemeral()).await,
    }
    let content = match errors != 0 {
        true => format!("Actioned `{actions_performed}` users, with `{errors}` errors!!"),
        false => format!("Actioned `{actions_performed}` users successfully!"),
    };

    ctx.respond(|r| r.content(content).ephemeral()).await
}
