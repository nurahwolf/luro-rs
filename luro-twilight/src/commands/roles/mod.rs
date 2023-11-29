use luro_framework::{Luro, LuroCommand};
use luro_model::{builders::EmbedBuilder, types::CommandResponse};
use tracing::{debug, info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::InteractionData,
    id::{marker::RoleMarker, Id},
};

use self::{blacklist::Blacklist, menu::Menu};

mod blacklist;
mod menu;

#[derive(CommandModel, CreateCommand)]
#[command(name = "roles", desc = "Manage your roles. Can also be used to setup a role menu")]
pub enum RoleCommands {
    #[command(name = "menu")]
    Menu(Menu),
    #[command(name = "blacklist")]
    Blacklist(Blacklist),
}

impl luro_framework::CreateLuroCommand for RoleCommands {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        match self {
            Self::Menu(cmd) => cmd.interaction_command(ctx).await,
            Self::Blacklist(cmd) => cmd.interaction_command(ctx).await,
        }
    }

    async fn interaction_component(
        self,
        ctx: luro_framework::ComponentInteraction,
        interaction: twilight_model::application::interaction::Interaction,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut embed = ctx.default_embed().await;
        let (mut guild, guild_data) = match ctx.guild {
            Some(ref guild) => match guild.data {
                Some(ref guild_data) => (guild.clone(), guild_data),
                None => {
                    return ctx
                        .respond(|r| r.content("I don't have data for this guild in my database, sorry!").ephemeral())
                        .await
                }
            },
            None => {
                return ctx
                    .respond(|r| r.content("You must run this command in a guild!").ephemeral())
                    .await
            }
        };

        let command = match interaction.data {
            Some(InteractionData::ApplicationCommand(ref data)) => data.clone(),
            _ => {
                return Err(anyhow::anyhow!(
                    "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                    interaction.data
                ))
            }
        };

        let data = Self::new(command)?;

        let mut member_requested_roles: Vec<Id<RoleMarker>> = vec![];
        let member_roles = ctx.database.member_fetch_roles(guild_data.guild_id, ctx.author.user_id).await?;
        let mut member_roles_sorted = member_roles.values().collect::<Vec<_>>();
        member_roles_sorted.sort();
        guild.roles.sort();

        let member_highest_role = match member_roles_sorted.first() {
            Some(role) => *role,
            None => {
                return ctx
                    .respond(|r| r.content("I don't have any roles recorded for you, sorry!").ephemeral())
                    .await
            }
        };

        for role in &ctx.data.values {
            if let Ok(role_id) = role.parse::<u64>() {
                member_requested_roles.push(Id::new(role_id))
            }
        }

        let mut current_roles = vec![];
        let mut blacklisted_roles = vec![];
        let mut too_high_roles = vec![];
        let mut roles_to_add = vec![];
        let mut roles_to_remove = vec![];

        for role in &guild.roles {
            if member_roles_sorted.contains(&role) {
                current_roles.push(role.role_id);

                // Member already has this role, but they have not requested to keep it
                if !guild_data.role_blacklist.contains(&role.role_id) && member_highest_role != role {
                    roles_to_remove.push(role.role_id)
                }
            }

            // The member requests this role
            if member_requested_roles.contains(&role.role_id) {
                if guild_data.role_blacklist.contains(&role.role_id) {
                    // The role is blacklisted
                    blacklisted_roles.push(role.role_id)
                } else if member_highest_role > role {
                    // The role is above the user in the role heirarchy
                    too_high_roles.push(role.role_id)
                } else if !member_roles_sorted.contains(&role) {
                    // The user does not have the role, add it to them
                    roles_to_add.push(role.role_id)
                }
            }
        }

        if !too_high_roles.is_empty() {
            let mut embed = embed.clone();
            add_role_field(&mut embed, &too_high_roles, "Selected Roles - Too High");
            if let Some(log_channel) = guild_data.moderator_actions_log_channel {
                warn!("User {} attempted to escalate their privileges", ctx.author.user_id);
                embed
                    .colour(luro_model::COLOUR_DANGER)
                    .title("Privilege Escalation Attempt")
                    .description(format!(
                        "The user <@{}> just attempted to give themselves higher roles than they should have",
                        ctx.author.user_id
                    ))
                    .author(|author| author.name(ctx.author.name()).icon_url(ctx.author.avatar_url()));
                ctx.send_message(&log_channel, |r| r.add_embed(embed.clone())).await?;
            }
        };

        if let Self::Menu(ref command) = data
            && &ctx.data.custom_id == "roles-button-rules"
        {
            roles_to_remove.drain(..);
            roles_to_add.push(command.rules.unwrap());
            if let Some(adult_role) = command.adult {
                for role in &member_roles {
                    if role.0 == &adult_role {
                        roles_to_remove.push(adult_role);
                    }
                }
            }
        }

        if let Self::Menu(ref command) = data
            && &ctx.data.custom_id == "roles-button-adult"
        {
            roles_to_remove.drain(..);
            roles_to_add.push(command.adult.unwrap());
            if let Some(rules) = command.rules {
                for role in &member_roles {
                    if role.0 == &rules {
                        roles_to_remove.push(rules);
                    }
                }
            }
        }

        if let Self::Menu(ref command) = data
            && &ctx.data.custom_id == "roles-button-bait"
        {
            roles_to_remove.drain(..);
            roles_to_add.push(command.bait.unwrap());
        }

        add_role_field(&mut embed, &current_roles, "Your current roles");
        add_role_field(&mut embed, &roles_to_add, "Roles I have added");
        add_role_field(&mut embed, &roles_to_remove, "Roles I have removed");
        add_role_field(&mut embed, &blacklisted_roles, "Blacklisted roles");
        add_role_field(&mut embed, &too_high_roles, "Roles above you");

        embed.author(|author| {
            author
                .name(format!("{} - Roles Updated", ctx.author.name()))
                .icon_url(ctx.author.avatar_url())
        });

        ctx.respond(|r| r.add_embed(embed.clone()).ephemeral()).await?;

        for role in roles_to_add {
            ctx.twilight_client
                .add_guild_member_role(guild_data.guild_id, ctx.author.user_id, role)
                .await?;
        }

        for role in roles_to_remove {
            ctx.twilight_client
                .remove_guild_member_role(guild_data.guild_id, ctx.author.user_id, role)
                .await?;
        }

        let member = ctx
            .twilight_client
            .guild_member(guild_data.guild_id, ctx.author.user_id)
            .await?
            .model()
            .await?;
        ctx.database.member_update((guild_data.guild_id, &member)).await?;

        info!("User {} just updated their roles", ctx.author.name());

        // if let Some(log_channel) = guild.catchall_log_channel {
        //     ctx.send_message(&log_channel, |r| r.add_embed(embed)).await?;
        // }

        Ok(CommandResponse::default())
    }
}

/// Appends a list of roles to an embed with the given name. If the passed array is empty, the field is not added
fn add_role_field<'a>(embed: &'a mut EmbedBuilder, roles: &[Id<RoleMarker>], name: &str) -> &'a mut EmbedBuilder {
    debug!("{:#?}", roles);
    let mut role_description = String::new();
    if !roles.is_empty() {
        for role_id in roles {
            role_description.push_str(&format!("- <@&{role_id}>\n"))
        }
        embed.create_field(name, &role_description, true);
    }
    embed
}
