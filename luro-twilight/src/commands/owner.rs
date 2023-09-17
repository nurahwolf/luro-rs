use async_trait::async_trait;
use luro_framework::command::LuroCommandTrait;
use luro_framework::responses::Response;
use luro_framework::{Framework, InteractionComponent, InteractionModal, LuroInteraction, CommandInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use luro_model::BOT_OWNERS;
use std::fmt::Write;
use tracing::warn;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::InteractionData;
use twilight_model::channel::message::component::SelectMenuType;
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::Id;

mod abuse;
mod assign;
mod clear_warnings;
mod commands;
// mod fakeban;
// mod flush;
// mod get_message;
// mod guilds;
// mod load_users;
// mod log;
pub mod mass_assign;
mod modify_role;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges!")]
pub enum Owner {
    #[command(name = "abuse")]
    Abuse(abuse::Abuse),
    #[command(name = "assign")]
    Assign(assign::Assign),
    #[command(name = "clear_warnings")]
    ClearWarning(clear_warnings::Warnings),
    #[command(name = "commands")]
    Commands(commands::Commands),
    #[command(name = "mass_assign")]
    MassAssign(mass_assign::MassAssign),
    #[command(name = "modify_role")]
    ModifyRole(modify_role::ModifyRole),
}

// pub enum OwnerCommands {
//     #[command(name = "config")]
//     #[command(name = "fakeban")]
//     #[command(name = "flush")]
//     #[command(name = "get_message")]
//     #[command(name = "guilds")]
//     #[command(name = "load_users")]
//     #[command(name = "log")]
//     Config(ConfigCommand),
//     FakeBan(FakeBan),
//     Flush(Flush),
//     GetMessage(OwnerGetMessage),
//     Guilds(OwnerGuildsCommand),
//     LoadUsers(OwnerLoadUsers),
//     Log(LogCommand),
// }

#[async_trait]
impl LuroCommandTrait for Owner {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: CommandInteraction<Self>,
    ) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        let interaction_author = ctx.author();

        let staff = match ctx.database.get_staff().await {
            Ok(data) => data.keys().copied().collect(),
            Err(why) => {
                warn!(why = ?why, "Failed to load staff from database, falling back to hardcoded staff members");
                BOT_OWNERS.to_vec()
            }
        };

        // If we don't have a match, bitch at the user
        if !staff.contains(&interaction_author.id) {
            return Response::NotOwner(
                &interaction_author.id,
                match ctx.command {
                    Self::Abuse(_) => "owner_abuse",
                    Self::Assign(_) => "owner_assign",
                    Self::ClearWarning(_) => "owner_clearwarning",
                    Self::Commands(_) => "owner_commands",
                    // Self::Config(_) => "owner_config",
                    // Self::FakeBan(_) => "owner_fakeban",
                    // Self::Flush(_) => "owner_save",
                    // Self::GetMessage(_) => "owner_getmessage",
                    // Self::Guilds(_) => "owner_guilds",
                    // Self::LoadUsers(_) => "owner_loadusers",
                    // Self::Log(_) => "owner_log",
                    Self::MassAssign(_) => "mass_assign",
                    Self::ModifyRole(_) => "owner_modify",
                },
            )
            .respond(&ctx)
            .await;
        }

        // We know the user is good, so call the appropriate subcommand.
        match ctx.command {
            Self::Abuse(_) => abuse::Abuse::handle_interaction(ctx).await,
            Self::Assign(_) => assign::Assign::handle_interaction(ctx).await,
            Self::ClearWarning(_) => clear_warnings::Warnings::handle_interaction(ctx).await,
            Self::Commands(_) => commands::Commands::handle_interaction(ctx).await,
            // Self::Config(_) => "owner_config",
            // Self::FakeBan(_) => "owner_fakeban",
            // Self::Flush(_) => "owner_save",
            // Self::GetMessage(_) => "owner_getmessage",
            // Self::Guilds(_) => "owner_guilds",
            // Self::LoadUsers(_) => "owner_loadusers",
            // Self::Log(_) => "owner_log",
            Self::MassAssign(_) => mass_assign::MassAssign::handle_interaction(ctx).await,
            Self::ModifyRole(_) => modify_role::ModifyRole::handle_interaction(ctx).await,
        }
    }

    async fn handle_modal<D: LuroDatabaseDriver>(ctx: Framework, interaction: InteractionModal) -> anyhow::Result<()> {
        let mut message_id = None;
        let mut channel_id = None;

        for row in &interaction.data.components {
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
            None => return interaction.respond(&ctx, |r| r.content("No message ID!").ephemeral()).await,
        };

        let channel_id = match channel_id {
            Some(channel_id) => Id::new(channel_id.parse()?),
            None => return interaction.respond(&ctx, |r| r.content("No channel ID!").ephemeral()).await,
        };

        let message = ctx.twilight_client.message(channel_id, message_id).await?.model().await?;
        let mut embed = message.embeds.first().unwrap().clone();

        for row in &interaction.data.components {
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

        interaction.respond(&ctx, |r| r.content("All done!").ephemeral()).await
    }

    async fn handle_component<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionComponent,
    ) -> anyhow::Result<()> {
        match interaction.data.custom_id.as_str() {
            "mass-assign-selector" => component_selector(ctx, interaction).await,
            "mass-assign-roles" | "mass-assign-remove" => component_roles(ctx, interaction).await,
            _ => interaction.respond(&ctx, |r| r.content("Unknown Command!").ephemeral()).await,
        }
    }
}

async fn component_selector<D: LuroDatabaseDriver>(ctx: Framework, interaction: InteractionComponent) -> anyhow::Result<()> {
    let mut roles_string = String::new();
    let guild_id = interaction.guild_id().unwrap();
    let mut data = None;
    let mut original = interaction.original_interaction::<D>().clone();

    while data.is_none() {
        match original.data {
            Some(InteractionData::MessageComponent(new_data)) => {
                data = Some(new_data);
                break;
            }
            _ => {
                if let Some(message) = original.message {
                    original = ctx.database.get_interaction(&message.id.to_string()).await?;
                }
            }
        }
    }

    let data = data.unwrap();

    let mut roles: Vec<Id<RoleMarker>> = data.values.iter().map(|role| Id::new(role.parse::<u64>().unwrap())).collect();

    // let guild = ctx.framework.twilight_cache.guild_members(guild_id).unwrap();
    let guild = ctx.twilight_client.guild_members(guild_id).limit(1000).await?.model().await?;
    let mut users = vec![];
    for member in guild.into_iter() {
        if let Ok(user) = ctx.database.get_user(&member.user.id).await {
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

    interaction.respond(&ctx, |response| {
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

async fn component_roles<D: LuroDatabaseDriver>(ctx: Framework, interaction: InteractionComponent) -> anyhow::Result<()> {
    let guild_id = interaction.guild_id().unwrap();

    let mut roles: Vec<Id<RoleMarker>> = interaction
        .data
        .values
        .iter()
        .map(|role| Id::new(role.parse::<u64>().unwrap()))
        .collect();

    // let guild = ctx.framework.twilight_cache.guild_members(guild_id).unwrap();
    let guild = ctx.twilight_client.guild_members(guild_id).limit(1000).await?.model().await?;
    let mut users = vec![];
    for member in guild.into_iter() {
        if let Ok(user) = ctx.database.get_user(&member.user.id).await {
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
    match interaction.data.custom_id.as_str() {
        "mass-assign-roles" => {
            for user in users {
                for role in &roles {
                    match ctx.twilight_client.add_guild_member_role(guild_id, user.id, *role).await {
                        Ok(_) => actions_performed += 1,
                        Err(_) => errors += 1,
                    };
                }
            }
        }
        "mass-assign-remove" => {
            for user in users {
                for role in &roles {
                    match ctx.twilight_client.remove_guild_member_role(guild_id, user.id, *role).await {
                        Ok(_) => actions_performed += 1,
                        Err(_) => errors += 1,
                    };
                }
            }
        }
        _ => return interaction.respond(&ctx, |r| r.content("It's fucked").ephemeral()).await,
    }
    let content = match errors != 0 {
        true => format!("Actioned `{actions_performed}` users, with `{errors}` errors!!"),
        false => format!("Actioned `{actions_performed}` users successfully!"),
    };

    interaction.respond(&ctx, |r| r.content(content).ephemeral()).await
}
