use luro_database::DatabaseInteraction;
use luro_framework::standard_response::Response;
use luro_framework::{CommandInteraction, ComponentInteraction, CreateLuroCommand, Luro, LuroCommand, ModalInteraction};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::Id;

// mod clear_warnings;
mod fakeban;
// mod flush;
// mod load_users;
// mod log;
mod abuse;
mod assign;
mod clear_marriage;
mod commands;
mod get_message;
mod guilds;
pub mod mass_assign;
mod modify_role;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges!")]
pub enum Owner {
    #[command(name = "abuse")]
    Abuse(abuse::Abuse),
    #[command(name = "assign")]
    Assign(assign::Assign),
    // #[command(name = "clear_warnings")]
    // ClearWarning(clear_warnings::Warnings),
    #[command(name = "clear_marriage")]
    ClearMarriage(clear_marriage::ClearMarriage),
    #[command(name = "commands")]
    Commands(commands::Commands),
    #[command(name = "mass_assign")]
    MassAssign(mass_assign::MassAssign),
    #[command(name = "modify_role")]
    ModifyRole(modify_role::ModifyRole),
    #[command(name = "get_message")]
    GetMessage(get_message::Message),
    #[command(name = "guilds")]
    Guilds(guilds::Guilds),
    #[command(name = "fakeban")]
    FakeBan(fakeban::FakeBan),


}

// pub enum OwnerCommands {
//     #[command(name = "config")]
//     #[command(name = "flush")]
//     #[command(name = "guilds")]
//     #[command(name = "load_users")]
//     #[command(name = "log")]
//     Config(ConfigCommand),
//     Flush(Flush),
//     LoadUsers(OwnerLoadUsers),
//     Log(LogCommand),
// }

impl std::fmt::Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Abuse(_) => "owner_abuse",
            Self::Assign(_) => "owner_assign",
            // Self::ClearWarning(_) => "owner_clearwarning",
            Self::Commands(_) => "owner_commands",
            Self::ClearMarriage(_) => "clear_marriage",
            // Self::Config(_) => "owner_config",
            Self::FakeBan(_) => "owner_fakeban",
            // Self::Flush(_) => "owner_save",
            Self::GetMessage(_) => "owner_getmessage",
            Self::Guilds(_) => "owner_guilds",
            // Self::LoadUsers(_) => "owner_loadusers",
            // Self::Log(_) => "owner_log",
            Self::MassAssign(_) => "mass_assign",
            Self::ModifyRole(_) => "owner_modify",
        };

        write!(f, "{}", name)
    }
}

impl CreateLuroCommand for Owner {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let mut authorised = false;
        for staff in ctx.database.get_staff().await? {
            if staff.user_id == ctx.author.user_id {
                authorised = true
            }
        }

        if !authorised {
            return ctx
                .response_simple(Response::NotOwner(&ctx.author.user_id(), &self.to_string()))
                .await;
        }

        match self {
            Self::Abuse(command) => command.interaction_command(ctx).await,
            Self::Assign(command) => command.interaction_command(ctx).await,
            // Self::ClearWarning(command) => command.interaction_command(ctx).await,
            Self::Commands(command) => command.interaction_command(ctx).await,
            Self::ClearMarriage(command) => command.interaction_command(ctx).await,
            // Self::Config(_) => "owner_config",
            Self::FakeBan(cmd) => cmd.interaction_command(ctx).await,
            // Self::Flush(_) => "owner_save",
            Self::GetMessage(command) => command.interaction_command(ctx).await,
            Self::Guilds(command) => command.interaction_command(ctx).await,
            // Self::LoadUsers(_) => "owner_loadusers",
            // Self::Log(_) => "owner_log",
            Self::MassAssign(command) => command.interaction_command(ctx).await,
            Self::ModifyRole(command) => command.interaction_command(ctx).await,
        }
    }

    async fn interaction_modal(ctx: ModalInteraction) -> anyhow::Result<()> {
        let mut message_id = None;
        let mut channel_id = None;

        for row in &ctx.data.components {
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

        let message = ctx.twilight_client.message(channel_id, message_id).await?.model().await?;
        let mut embed = message.embeds.first().unwrap().clone();

        for row in &ctx.data.components {
            for component in &row.components {
                if let Some(ref value) = component.value && component.custom_id.as_str() == "embed-title" {
                    embed.title = Some(value.clone());
                }
                if let Some(ref value) = component.value && component.custom_id.as_str() == "embed-description" {
                    embed.description = Some(value.clone());
                }
            }
        }

        ctx.twilight_client
            .update_message(channel_id, message_id)
            .embeds(Some(vec![embed]).as_deref())
            .await?;

        ctx.respond(|r| r.content("All done!").ephemeral()).await
    }

    async fn interaction_component(self, ctx: ComponentInteraction, _invoking_interaction: DatabaseInteraction) -> anyhow::Result<()> {
        match ctx.data.custom_id.as_str() {
            "mass-assign-selector" => component_selector(ctx).await,
            "mass-assign-roles" | "mass-assign-remove" => component_roles(ctx).await,
            _ => ctx.respond(|r| r.content("Unknown Command!").ephemeral()).await,
        }
    }
}

async fn component_selector(ctx: ComponentInteraction) -> anyhow::Result<()> {
    let guild = match &ctx.guild {
        Some(guild) => guild,
        None => return ctx.response_simple(luro_framework::Response::NotGuild).await,
    };
    let guild_roles = ctx.get_guild_roles(&guild.guild_id(), false).await?;

    let mut roles_string = String::new();
    for role in &guild_roles {
        writeln!(roles_string, "- <@&{}>", role.id)?;
    }

    // let roles: Vec<Id<RoleMarker>> = ctx.data.values.iter().map(|role| Id::new(role.parse::<u64>().unwrap())).collect();

    // let mut users = vec![];
    // for member in guild.into_iter() {
    //     if let Ok(user) = ctx.fetch_user(&member.user.id).await {
    //         users.push(user)
    //     }
    // }

    // match roles.is_empty() {
    //     true => roles.push(guild_id.cast()),
    //     false => users.retain(|user| {
    //         let mut found = false;
    //         match user.guilds.get(&guild_id) {
    //             Some(guild_data) => {
    //                 for role in &roles {
    //                     if guild_data.role_ids.contains(role) {
    //                         found = true
    //                     }
    //                 }
    //             }
    //             None => found = false,
    //         };
    //         found
    //     }),
    // };

    // ctx.respond( |response| {
    //     {
    //         response
    //             .content(format!("Found `{}` users with the role(s):\n{roles_string}\nFirst Menu: The roles to apply\nSecond Menu: The roles to remove", users.len()))
    //             .components(|components| {
    //                 components
    //                     .action_row(|row| {
    //                         row.component(|component| {
    //                             component.select_menu(|menu| {
    //                                 menu.custom_id("mass-assign-roles")
    //                                     .kind(SelectMenuType::Role)
    //                                     .max_values(25)
    //                                     .min_values(1)
    //                             })
    //                         })
    //                     })
    //                     .action_row(|row| {
    //                         row.component(|component| {
    //                             component.select_menu(|menu| {
    //                                 menu.custom_id("mass-assign-remove")
    //                                     .kind(SelectMenuType::Role)
    //                                     .max_values(25)
    //                                     .min_values(1)
    //                             })
    //                         })
    //                     })
    //             })
    //     }
    //     .ephemeral()
    // })
    // .await

    ctx.respond(|r| r.content("To implement")).await
}

async fn component_roles(ctx: ComponentInteraction) -> anyhow::Result<()> {
    // let guild = match &ctx.guild {
    //     Some(guild) => guild,
    //     None => return ctx.response_simple(luro_framework::Response::NotGuild).await,
    // };

    // let roles: Vec<Id<RoleMarker>> = ctx.data.values.iter().map(|role| Id::new(role.parse::<u64>().unwrap())).collect();

    // // let guild = ctx.cache.guild_members(guild_id).unwrap();
    // // let mut members = vec![];
    // // for member in guild.iter() {
    // //     if let Ok(user) = ctx.fetch_member(member, &guild_id).await {
    // //         members.push(user)
    // //     }
    // // }

    // // match roles.is_empty() {
    // //     true => roles.push(guild_id.cast()),
    // //     false => members.retain(|member| {
    // //         let mut found = false;
    // //         for role in member.

    // //         match user.guilds.get(&guild_id) {
    // //             Some(guild_data) => {
    // //                 for role in &roles {
    // //                     if guild_data.role_ids.contains(role) {
    // //                         found = true
    // //                     }
    // //                 }
    // //             }
    // //             None => found = false,
    // //         };
    // //         found
    // //     }),
    // // };

    // let mut actions_performed = 0;
    // let mut errors = 0;
    // match ctx.data.custom_id.as_str() {
    //     // "mass-assign-roles" => {
    //     //     for user in members {
    //     //         for role in &roles {
    //     //             match ctx.twilight_client.add_guild_member_role(guild.guild_id(), user.user_id(), *role).await {
    //     //                 Ok(_) => actions_performed += 1,
    //     //                 Err(_) => errors += 1,
    //     //             };
    //     //         }
    //     //     }
    //     // }
    //     // "mass-assign-remove" => {
    //     //     for user in members {
    //     //         for role in &roles {
    //     //             match ctx.twilight_client.remove_guild_member_role(guild.guild_id(), user.user_id(), *role).await {
    //     //                 Ok(_) => actions_performed += 1,
    //     //                 Err(_) => errors += 1,
    //     //             };
    //     //         }
    //     //     }
    //     // }
    //     _ => return ctx.respond(|r| r.content("It's fucked").ephemeral()).await,
    // }
    // let content = match errors != 0 {
    //     true => format!("Actioned `{actions_performed}` users, with `{errors}` errors!!"),
    //     false => format!("Actioned `{actions_performed}` users successfully!"),
    // };

    let content = "WIP!";
    ctx.respond(|r| r.content(content).ephemeral()).await
}
