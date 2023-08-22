use luro_builder::embed::EmbedBuilder;
use luro_model::{legacy::role_ordering::RoleOrdering, COLOUR_DANGER};
use std::fmt::Write;
use tracing::{debug, info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    channel::message::component::{ButtonStyle, SelectMenuType},
    id::{marker::RoleMarker, Id}
};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_model::database::drivers::LuroDatabaseDriver;

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
        // Variables
        let mut roles_to_remove_string = String::new();
        let mut roles_to_remove = vec![];
        let mut roles_to_add_string = String::new();
        let mut roles_to_add = vec![];
        let mut blacklisted_roles = vec![];

        let mut embed = EmbedBuilder::default();
        let accent_colour = ctx.accent_colour().await;
        let luro_user = ctx
            .framework
            .database
            .get_user(&ctx.interaction.author_id().unwrap(), &ctx.framework.twilight_client)
            .await?;

        let guild_id = ctx.interaction.guild_id.unwrap();
        let member = ctx
            .framework
            .twilight_client
            .guild_member(guild_id, luro_user.id)
            .await?
            .model()
            .await?;
        let guild = ctx.framework.twilight_client.guild(guild_id).await?.model().await?;

        // Get and sort user's roles
        let selected: Vec<Id<RoleMarker>> = data.values.iter().map(|role| Id::new(role.parse::<u64>().unwrap())).collect();
        let mut raw_user_roles = vec![];
        let mut selected_roles = vec![];
        let guild_settings = ctx
            .framework
            .database
            .get_guild(&guild_id, &ctx.framework.twilight_client)
            .await?;
        for guild_role in guild.roles.clone() {
            for member_role in &member.roles {
                if guild_settings.assignable_role_blacklist.contains(&guild_role.id) {
                    continue;
                }

                if member_role == &guild_role.id {
                    raw_user_roles.push(guild_role.clone())
                }
            }

            for role in &selected {
                if guild_settings.assignable_role_blacklist.contains(role) {
                    if !blacklisted_roles.contains(role) {
                        blacklisted_roles.push(*role)
                    }
                    continue;
                }

                if role == &guild_role.id {
                    selected_roles.push(guild_role.clone())
                }
            }
        }
        // Turn them into ordered roles
        let mut user_roles: Vec<_> = raw_user_roles.iter().map(RoleOrdering::from).collect();
        user_roles.sort_by(|a, b| b.cmp(a));
        let mut selected_roles: Vec<_> = selected_roles.iter().map(RoleOrdering::from).collect();
        selected_roles.sort_by(|a, b| b.cmp(a));
        let highest_role = *user_roles.first().unwrap();
        info!("Highest Role - {}", highest_role.id);

        // List current roles
        let mut current_role_list = String::new();
        let mut current_roles = vec![];
        for role in &user_roles {
            current_roles.push(*role);
            writeln!(current_role_list, "- <@&{}>", role.id)?
        }
        embed.create_field("Current Roles", &current_role_list, false);

        // List selected roles
        let mut too_high_role_list = String::new();
        for role in &selected_roles {
            match role.position > highest_role.position {
                true => writeln!(too_high_role_list, "- <@&{}>", role.id)?,
                false => {
                    if current_roles.contains(role) {
                        continue;
                    }

                    roles_to_add.push(role.id);
                    writeln!(roles_to_add_string, "- <@&{}>", role.id)?
                }
            }
        }

        if !too_high_role_list.is_empty() {
            embed.create_field("Selected Roles - Too High", &too_high_role_list, false);
            if let Some(log_channel) = guild_settings.moderator_actions_log_channel {
                // TODO: Remove hardcoded channel
                warn!("User {} attempted to escalate their privileges", luro_user.id);
                ctx.framework
                    .send_message(&log_channel, |r| {
                        r.embed(|embed| {
                            embed
                                .colour(COLOUR_DANGER)
                                .title("Privilege Escalation Attempt")
                                .description(format!(
                                    "The user <@{}> just attempted to give themselves higher roles than they should have",
                                    luro_user.id
                                ))
                                .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()))
                                .create_field("Roles Attempted", &too_high_role_list, false)
                        })
                        .content("<@&1037361382410170378>")
                    })
                    .await?;
            }
        }

        // Check if we need to remove any roles
        for role in &user_roles {
            if highest_role == *role {
                continue;
            }

            if !selected_roles.contains(role) {
                roles_to_remove.push(role.id);
                writeln!(roles_to_remove_string, "- <@&{}>", role.id)?
            }
        }

        if let Self::Menu(command) = self {
            if &data.custom_id == "rules-button" {
                roles_to_remove.drain(..);
                roles_to_remove_string.drain(..);
                let rules_role = command.rules.unwrap();
                if let Some(adult_role) = command.adult {
                    for role in &user_roles {
                        if role.id == adult_role {
                            roles_to_remove.push(adult_role);
                            writeln!(roles_to_remove_string, "- <@&{}> - (Rules Role)", adult_role)?;
                        }
                    }
                }
                roles_to_add.push(rules_role);
                writeln!(roles_to_add_string, "- <@&{}> - (Rules Button)", rules_role)?;
            }
            if &data.custom_id == "adult-button" {
                roles_to_remove.drain(..);
                roles_to_remove_string.drain(..);
                let adult_role = command.adult.unwrap();
                if let Some(rules_role) = command.rules {
                    for role in &user_roles {
                        if role.id == rules_role {
                            roles_to_remove.push(rules_role);
                            writeln!(roles_to_remove_string, "- <@&{}> - (Rules Role)", rules_role)?;
                        }
                    }
                }
                roles_to_add.push(adult_role);
                writeln!(roles_to_add_string, "- <@&{}> - (Adult Button)", adult_role)?;
            }
            if &data.custom_id == "bait-button" {
                roles_to_remove.drain(..);
                roles_to_remove_string.drain(..);
                roles_to_add.push(command.bait.unwrap());
                writeln!(roles_to_add_string, "- <@&{}> - (Bait Button)", command.bait.unwrap())?;
            }
            if !roles_to_add_string.is_empty() {
                embed.create_field("Roles to add", &roles_to_add_string, false);
            }
            if !roles_to_remove_string.is_empty() {
                embed.create_field("Roles to remove", &roles_to_remove_string, false);
            }

            let mut blacklisted_roles_list = String::new();
            for role in &blacklisted_roles {
                writeln!(blacklisted_roles_list, "- <@&{}>", role)?;
            }

            if !blacklisted_roles_list.is_empty() {
                embed.create_field("Blacklisted Roles", &blacklisted_roles_list, false);
            }
        }

        debug!("{:#?}", roles_to_add);
        debug!("{:#?}", roles_to_remove);
        debug!("{:#?}", blacklisted_roles);

        embed
            .colour(accent_colour)
            .title("Roles Updated")
            .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()));

        ctx.respond(|r| r.add_embed(embed.clone()).ephemeral()).await?;

        for role in roles_to_add {
            ctx.framework
                .twilight_client
                .add_guild_member_role(guild_id, luro_user.id, role)
                .await?;
        }

        for role in roles_to_remove {
            ctx.framework
                .twilight_client
                .remove_guild_member_role(guild_id, luro_user.id, role)
                .await?;
        }

        info!("User {} just updated their roles", luro_user.name());

        if let Some(log_channel) = guild_settings.catchall_log_channel {
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
        let luro_user = ctx
            .framework
            .database
            .get_user(&interaction_author, &ctx.framework.twilight_client)
            .await?;

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
        let _member = &ctx.interaction.member.clone().unwrap();
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
