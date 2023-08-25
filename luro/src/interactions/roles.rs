use anyhow::Context;
use luro_builder::embed::EmbedBuilder;
use tracing::{debug, info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    channel::message::component::{ButtonStyle, SelectMenuType},
    id::{marker::RoleMarker, Id}
};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_model::{database::drivers::LuroDatabaseDriver, COLOUR_DANGER};

use self::blacklist::Blacklist;

mod blacklist;

#[derive(CommandModel, CreateCommand)]
#[command(name = "roles", desc = "Manage your roles. Can also be used to setup a role menu")]
pub enum RoleCommands {
    #[command(name = "menu")]
    Menu(Menu),
    #[command(name = "blacklist")]
    Blacklist(Blacklist)
}

impl LuroCommand for RoleCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        match self {
            Self::Menu(command) => command.run_command(ctx).await,
            Self::Blacklist(command) => command.run_command(ctx).await
        }
    }

    async fn handle_component<D: LuroDatabaseDriver>(
        self,
        data: Box<MessageComponentInteractionData>,
        ctx: LuroSlash<D>
    ) -> anyhow::Result<()> {
        let raw_selected_roles: Vec<Id<RoleMarker>> =
            data.values.iter().map(|role| Id::new(role.parse::<u64>().unwrap())).collect();
        let guild_id = ctx.interaction.guild_id.unwrap();

        let mut blacklisted_roles = vec![];
        let mut roles_to_add = vec![];
        let mut roles_to_remove = vec![];
        let mut selected_roles = vec![];
        let mut existing_roles = vec![];
        let mut too_high_role = vec![];

        let mut embed = ctx.default_embed().await;
        let guild = ctx.framework.database.get_guild(&guild_id).await?;
        let mut user = ctx.framework.database.get_user(&ctx.interaction.author_id().unwrap()).await?;
        let user_roles = guild.user_roles(&user);
        let user_highest_role = match guild.user_highest_role(&user) {
            Some(role) => role,
            None => {
                let member = ctx
                    .framework
                    .twilight_client
                    .guild_member(guild_id, user.id)
                    .await?
                    .model()
                    .await?;
                user.update_member(&guild_id, &member);
                ctx.framework.database.save_user(&user.id, &user).await?;
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

                // Add this to a list of selected roles to add to a user
                selected_roles.push(*role_id)
            }
        }

        for user_role in guild.user_roles(&user) {
            // Don't modify blacklisted roles
            if guild.assignable_role_blacklist.contains(&user_role.id) {
                continue;
            }

            // If this role is not one they selected, add it to the list to be removed
            match selected_roles.contains(&user_role.id) {
                true => existing_roles.push(user_role.id),
                false => {
                    // Don't remove their higest role!
                    if user_highest_role.1 == user_role.id {
                        continue;
                    }

                    roles_to_remove.push(user_role.id)
                }
            }
        }

        if !too_high_role.is_empty() {
            add_role_field(&mut embed, &too_high_role, "Selected Roles - Too High");
            if let Some(log_channel) = guild.moderator_actions_log_channel {
                // TODO: Remove hardcoded channel
                warn!("User {} attempted to escalate their privileges", user.id);
                embed
                    .colour(COLOUR_DANGER)
                    .title("Privilege Escalation Attempt")
                    .description(format!(
                        "The user <@{}> just attempted to give themselves higher roles than they should have",
                        user.id
                    ))
                    .author(|author| author.name(user.name()).icon_url(user.avatar()));
                ctx.framework
                    .send_message(&log_channel, |r| {
                        r.add_embed(embed.clone())
                            // TODO: Fix hard coded staff ping
                            .content("<@&1037361382410170378>")
                    })
                    .await?;
            }
        };

        if let Self::Menu(ref command) = self && &data.custom_id == "rules-button" {
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

        if let Self::Menu(ref command) = self && &data.custom_id == "adult-button" {
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

        if let Self::Menu(ref command) = self && &data.custom_id == "bait-button" {
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

        ctx.respond(|r| r.add_embed(embed.clone()).ephemeral()).await?;

        for role in roles_to_add {
            ctx.framework
                .twilight_client
                .add_guild_member_role(guild_id, user.id, role)
                .await?;
        }

        for role in roles_to_remove {
            ctx.framework
                .twilight_client
                .remove_guild_member_role(guild_id, user.id, role)
                .await?;
        }

        info!("User {} just updated their roles", user.name());

        if let Some(log_channel) = guild.catchall_log_channel {
            ctx.framework.send_message(&log_channel, |r| r.add_embed(embed)).await?;
        }

        Ok(())
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "menu", desc = "Show a role menu, for easily selecting roles")]
pub struct Menu {
    /// Customise the embed description. Leave blank for the default
    description: Option<String>,
    /// Customise the embed title. Leave blank for none
    title: Option<String>,
    /// Role to give to those who agree to the rules (e.g. Minor)
    rules: Option<Id<RoleMarker>>,
    /// The button's label. Defaults to the role name
    rules_label: Option<String>,
    /// Role to give to those who agree to the rules AND are over 18
    adult: Option<Id<RoleMarker>>,
    /// The button's label. Defaults to the role name
    adult_label: Option<String>,
    /// Role to give to those that clicked the funny button
    bait: Option<Id<RoleMarker>>,
    /// The button's label. Defaults to the role name
    bait_label: Option<String>
}

impl LuroCommand for Menu {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let interaction_author = ctx.interaction.author_id().unwrap();
        let luro_user = ctx.framework.database.get_user(&interaction_author).await?;

        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live

        for (id, _) in ctx.framework.database.get_staff().await? {
            if interaction_author == id {
                owner_match = true
            }
        }

        if !owner_match {
            return ctx
                .not_owner_response(&interaction_author, &ctx.interaction.guild_id, "role-menu")
                .await;
        }

        // SAFETY: This command can only be used in guilds
        let add_buttons = self.rules.is_some() || self.adult.is_some() || self.bait.is_some();

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|response| {
            response
                .embed(|embed| {
                    embed
                        .colour(accent_colour)
                        .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()));
                    match self.description {
                        Some(description) => embed.description(description),
                        None => embed.description("Select the roles you want")
                    };
                    if let Some(title) = self.title {
                        embed.title(title);
                    }
                    embed
                })
                .components(|components| {
                    components.action_row(|row| {
                        row.component(|component| {
                            component.select_menu(|menu| menu.custom_id("role-menu").kind(SelectMenuType::Role).max_values(25))
                        })
                    })
                });

            if add_buttons {
                response.components(|components| {
                    components.action_row(|row| {
                        if let Some(role) = self.rules {
                            let role = ctx.framework.twilight_cache.role(role).unwrap().clone();
                            row.button(|button| {
                                button.custom_id("rules-button").style(ButtonStyle::Primary);
                                match self.rules_label {
                                    Some(label) => button.label(label),
                                    None => button.label(role.name.clone())
                                };
                                button
                            });
                        }
                        if let Some(role) = self.adult {
                            let role = ctx.framework.twilight_cache.role(role).unwrap().clone();
                            row.button(|button| {
                                button.custom_id("adult-button").style(ButtonStyle::Primary);
                                match self.adult_label {
                                    Some(label) => button.label(label),
                                    None => button.label(role.name.clone())
                                };
                                button
                            });
                        }
                        if let Some(role) = self.bait {
                            let role = ctx.framework.twilight_cache.role(role).unwrap().clone();
                            row.button(|button| {
                                button.custom_id("bait-button").style(ButtonStyle::Danger);
                                match self.bait_label {
                                    Some(label) => button.label(label),
                                    None => button.label(role.name.clone())
                                };
                                button
                            });
                        }
                        row
                    })
                });
            }
            response
        })
        .await
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
                false => role_description.push_str(&format!(", <@&{}>", role))
            }
        }
        embed.create_field(name, &role_description, false);
    }
    embed
}
