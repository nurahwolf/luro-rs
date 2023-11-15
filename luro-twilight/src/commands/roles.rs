use crate::commands::anyhow;
use anyhow::Context;
use async_trait::async_trait;
use luro_builder::embed::EmbedBuilder;
use luro_framework::{
    command::LuroCommandTrait,
    Framework, InteractionCommand, InteractionComponent, LuroInteraction, CommandInteraction,
};
use luro_model::{database_driver::LuroDatabaseDriver, COLOUR_DANGER};
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

#[async_trait]
impl LuroCommandTrait for RoleCommands {
    async fn handle_interaction(
        ctx: CommandInteraction<Self>,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
        match ctx.command {
            Self::Menu(_command) => menu::Menu::handle_interaction(ctx).await,
            Self::Blacklist(_command) => blacklist::Blacklist::handle_interaction(ctx).await,
        }
    }

    async fn handle_component(
        ctx: Framework,
        interaction: InteractionComponent,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut message = interaction.message.clone();
        let mut original_interaction = interaction.original.clone();
        let mut new_id = true;

        while new_id {
            original_interaction = ctx.database.get_interaction(&message.id.to_string()).await?;

            new_id = match original_interaction.message {
                Some(ref new_message) => {
                    message = new_message.clone();
                    true
                }
                None => false,
            }
        }

        let command = match original_interaction.data {
            Some(InteractionData::ApplicationCommand(ref data)) => data.clone(),
            _ => {
                return Err(anyhow!(
                    "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                    interaction.data
                ))
            }
        };

        let data = Self::new(command)?;
        let raw_selected_roles: Vec<Id<RoleMarker>> = interaction
            .data
            .values
            .iter()
            .map(|role| Id::new(role.parse::<u64>().unwrap()))
            .collect();
        let guild_id = interaction.guild_id.unwrap();

        let mut blacklisted_roles = vec![];
        let mut roles_to_add = vec![];
        let mut roles_to_remove = vec![];
        let mut selected_roles = vec![];
        let mut existing_roles = vec![];
        let mut too_high_role = vec![];

        let mut embed = interaction.default_embed(&ctx).await;
        let guild = ctx.database.get_guild(&guild_id).await?;
        let mut user = ctx.database.get_user(&interaction.author_id()).await?;
        let user_roles = guild.user_roles(&user);
        let user_highest_role = match guild.user_highest_role(&user) {
            Some(role) => role,
            None => {
                let member = ctx.twilight_client.guild_member(guild_id, user.id).await?.model().await?;
                user.update_member(&guild_id, &member);
                ctx.database.modify_user(&user.id, &user).await?;
                guild
                    .user_highest_role(&user)
                    .context("Expected to get user's highest role")?
            }
        };

        // For each role in guild
        for (position, role_id) in &guild.role_positions {
            // If the user selected this role
            if raw_selected_roles.contains(role_id) {
                // If the role is higher than theirs
                if &user_highest_role.0 > position {
                    too_high_role.push(*role_id);
                    continue;
                }

                // If the role is in the guild's blacklist
                if guild.assignable_role_blacklist.contains(role_id) {
                    blacklisted_roles.push(*role_id);
                    continue;
                }

                selected_roles.push(*role_id);
            }
        }

        for user_role in guild.user_roles(&user) {
            // Don't modify blacklisted roles
            if guild.assignable_role_blacklist.contains(&user_role.id) {
                existing_roles.push(user_role.id);
                continue;
            }

            // If this role is not one they selected, add it to the list to be removed
            if !selected_roles.contains(&user_role.id) {
                // Don't remove their higest role!
                if user_highest_role.1 == user_role.id {
                    continue;
                }

                roles_to_remove.push(user_role.id)
            }
        }

        let user_role_ids: Vec<_> = guild.user_roles(&user).iter().map(|x| x.id).collect();
        for role in &selected_roles {
            if !user_role_ids.contains(role) {
                roles_to_add.push(*role)
            }
        }

        if !too_high_role.is_empty() {
            add_role_field(&mut embed, &too_high_role, "Selected Roles - Too High");
            if let Some(log_channel) = guild.moderator_actions_log_channel {
                warn!("User {} attempted to escalate their privileges", user.id);
                embed
                    .colour(COLOUR_DANGER)
                    .title("Privilege Escalation Attempt")
                    .description(format!(
                        "The user <@{}> just attempted to give themselves higher roles than they should have",
                        user.id
                    ))
                    .author(|author| author.name(user.name()).icon_url(user.avatar()));
                ctx.send_message(&log_channel, |r| r.add_embed(embed.clone())).await?;
            }
        };

        if let Self::Menu(ref command) = data && &interaction.data.custom_id == "rules-button" {
            roles_to_remove.drain(..);
            roles_to_add.push(command.rules.unwrap());
            if let Some(adult_role) = command.adult {
                for role in &user_roles {
                    if role.id == adult_role {
                        roles_to_remove.push(adult_role);
                    }
                }
            }
        }

        if let Self::Menu(ref command) = data && &interaction.data.custom_id == "adult-button" {
            roles_to_remove.drain(..);
            roles_to_add.push(command.adult.unwrap());
            if let Some(rules) = command.rules {
                for role in &user_roles {
                    if role.id == rules {
                        roles_to_remove.push(rules);
                    }
                }
            }
        }

        if let Self::Menu(ref command) = data && &interaction.data.custom_id == "bait-button" {
            roles_to_remove.drain(..);
            roles_to_add.push(command.bait.unwrap());
        }

        add_role_field(&mut embed, &existing_roles, "User Roles");
        add_role_field(&mut embed, &selected_roles, "Selected Roles");
        add_role_field(&mut embed, &roles_to_add, "Added Roles");
        add_role_field(&mut embed, &roles_to_remove, "Removed Roles");
        add_role_field(&mut embed, &blacklisted_roles, "Blacklisted Roles");

        embed
            .title("Roles Updated")
            .author(|author| author.name(user.name()).icon_url(user.avatar()));

        interaction.respond(&ctx, |r| r.add_embed(embed.clone()).ephemeral()).await?;

        for role in roles_to_add {
            ctx.twilight_client.add_guild_member_role(guild_id, user.id, role).await?;
        }

        for role in roles_to_remove {
            ctx.twilight_client.remove_guild_member_role(guild_id, user.id, role).await?;
        }

        info!("User {} just updated their roles", user.name());

        if let Some(log_channel) = guild.catchall_log_channel {
            ctx.send_message(&log_channel, |r| r.add_embed(embed)).await?;
        }

        Ok(())
    }
}

/// Appends a list of roles to an embed with the given name. If the passed array is empty, the field is not added
fn add_role_field<'a>(embed: &'a mut EmbedBuilder, roles: &[Id<RoleMarker>], name: &str) -> &'a mut EmbedBuilder {
    debug!("{:#?}", roles);
    let mut role_description = String::new();
    if !roles.is_empty() {
        for role in roles {
            match role_description.is_empty() {
                true => role_description.push_str(&format!("<@&{}>", role)),
                false => role_description.push_str(&format!(", <@&{}>", role)),
            }
        }
        embed.create_field(name, &role_description, false);
    }
    embed
}
