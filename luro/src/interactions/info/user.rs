use anyhow::Context;
use luro_builder::response::LuroResponse;

use std::{collections::btree_map::Entry, fmt::Write, time::Duration};

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    http::{attachment::Attachment, interaction::InteractionResponseType},
    id::{marker::GenericMarker, Id}
};
use twilight_util::snowflake::Snowflake;

use crate::interaction::LuroSlash;
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    legacy::role_ordering::RoleOrdering,
    user::{actions_type::UserActionType, member::LuroMember}
};

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand)]
#[command(name = "user", desc = "Information about a user")]
pub struct InfoUser {
    /// The user to get, gets yourself if not specified
    user: Option<ResolvedUser>,
    /// Optionally try to get a user from a different guild
    guild: Option<Id<GenericMarker>>,
    /// Just show user details, not guild details
    user_only: Option<bool>,
    /// Hide the user's avatar, so that there is more space for the details
    hide_avatar: Option<bool>,
    /// Set this if you want a copy of your data.
    gdpr_export: Option<bool>
}

impl LuroCommand for InfoUser {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let response_type = InteractionResponseType::DeferredChannelMessageWithSource;
        ctx.acknowledge_interaction(self.gdpr_export.unwrap_or_default()).await?;
        // The user we are interested in is the interaction author, unless a user was specified
        let user_id = match self.user {
            Some(ref user) => user.resolved.id,
            None => ctx.interaction.author_id().unwrap()
        };
        let mut luro_user = ctx
            .framework
            .database
            .get_user(&user_id, &ctx.framework.twilight_client)
            .await?;
        let user_timestamp = Duration::from_millis(user_id.timestamp().unsigned_abs());

        let mut response = LuroResponse::default();
        let mut embed = ctx.default_embed().await;
        let mut description = String::new();
        let mut timestamp = format!("- Joined discord on <t:{0}> - <t:{0}:R>\n", user_timestamp.as_secs());

        embed.author(|author| {
            author
                .name(format!("{} - {}", luro_user.name, luro_user.id))
                .icon_url(luro_user.avatar())
        });
        if let Some(hide_avatar) = self.hide_avatar && hide_avatar {
        } else {
            embed.thumbnail(|thumbnail|thumbnail.url(luro_user.avatar()));
        }

        writeln!(description, "<@{}>", luro_user.id)?;
        if let Some(accent_color) = luro_user.accent_color {
            embed.colour(accent_color);
            writeln!(description, "- Accent Colour: `{accent_color:X}`")?;
        }
        if let Some(email) = &luro_user.email {
            writeln!(description, "- Email: `{}`", email)?;
        }
        if let Some(locale) = &luro_user.locale {
            writeln!(description, "- Locale: `{}`", locale)?;
        }
        if luro_user.mfa_enabled {
            writeln!(description, "- MFA Enabled: `true`")?;
        }
        if luro_user.system {
            writeln!(description, "- System Account: `true`")?;
        }
        if luro_user.verified {
            writeln!(description, " - Verified Account: `true`")?;
        }
        if luro_user.bot {
            writeln!(description, " - Bot: `true`")?;
        }
        if let Some(ref banner) = luro_user.banner() {
            embed.image(|i| i.url(banner));
        }

        // Some additional details if we are a guild
        let guild_id = match self.guild {
            Some(guild_specified) => Some(Id::new(guild_specified.get())),
            None => ctx.interaction.guild_id
        };

        if let Some(guild_id) = guild_id && !self.user_only.is_some_and(|user_only| user_only) {
            let guild = ctx.framework.twilight_client.guild(guild_id).await?.model().await?;
            let member = ctx.framework.twilight_cache.member(guild_id, user_id).context("Expected to find member in cache")?;

            let mut guild_information = String::new();
            let mut role_list = String::new();
            let mut user_roles = vec![];
            for member_role in member.roles() {
                for guild_role in guild.roles.clone() {
                    if member_role == &guild_role.id {
                        user_roles.push(guild_role)
                    }
                }
            }

            let mut user_roles_modified: Vec<_> = user_roles.iter().map(RoleOrdering::from).collect();
            user_roles_modified.sort_by(|a, b| b.cmp(a));

            match luro_user.guilds.entry(guild_id) {
                Entry::Vacant(entry) => {
                    let entry = entry.insert(LuroMember::from(&ctx.framework.twilight_client.guild_member(guild_id, user_id).await?.model().await?));
                    for role in user_roles {
                        entry.role_ids.push(role.id)
                    }
                },
                Entry::Occupied(mut entry) => {
                    let entry = entry.get_mut();
                    for role in user_roles {
                        entry.role_ids.push(role.id)
                    }
                },
            };

            ctx.framework.database.save_user(&user_id, &luro_user).await?;

            for role in &user_roles_modified {
                if role_list.is_empty() {
                    write!(role_list, "<@&{}>", role.id)?;
                    continue;
                };
                write!(role_list, ", <@&{}>", role.id)?
            }

            if let Some(role) = user_roles_modified.first() {
                if role.colour != 0 {
                    embed.colour(role.colour);
                }
            }
            writeln!(guild_information, "- Roles ({}): {role_list}", user_roles_modified.len())?;
            timestamp.push_str(format!("- Joined this server at <t:{0}> - <t:{0}:R>\n", member.joined_at().as_secs()).as_str());
            if let Some(member_timestamp) = member.premium_since() {
                timestamp.push_str(format!("- Boosted this server since <t:{0}> - <t:{0}:R>", member_timestamp.as_secs()).as_str());
            }
            if let Some(nickname) = member.nick() {
                writeln!(guild_information, "- Nickname: `{nickname}`")?;            
            }
            if member.deaf().unwrap_or_default() {
                writeln!(guild_information, "- Deafened: `true`")?;            
            }
            if member.mute().unwrap_or_default() {
                writeln!(guild_information, "- Muted: `true`")?;            
            }
            if member.pending() {
                writeln!(guild_information, "- Pending: `true`")?;            
            }
            if let Some(timestamp) = member.communication_disabled_until() {
                writeln!(guild_information, "- Timed out until: <t:{}:R>", timestamp.as_secs())?;            
            }
            // TODO: Once member_banner is a thing in [Member]
            // if let Some(banner) = get_member_banner(&member, guild_id, user) {
            //     embed = embed.image(ImageSource::url(banner)?)
            // }
            embed.author(|author| author.name(luro_user.member_name(&Some(guild_id))).icon_url(luro_user.guild_avatar(&guild_id)));
            match guild_information.len() > 1024 {
                true => {writeln!(description, "\n**Guild Information**\n{guild_information}")?;},
                false => {embed.create_field("Guild Information", &guild_information, false);},
            }
        }

        // USER DATA SECTION
        let mut user_data_description = String::new();
        {
            if let Some(export) = self.gdpr_export && export {
                if let Some(user_specified) = self.user {
                    // TODO: Add privilege esc tally to the person
                    return ctx.respond(|r|r.content(format!("Hey <@{}>! <@{}> is being a cunt and trying to steal your data.", user_specified.resolved.id, ctx.interaction.author_id().unwrap())).response_type(response_type)).await
                }
                response.attachments = Some(vec![Attachment::from_bytes(
                    format!("gdpr-export-{}.txt", ctx.interaction.author_id().unwrap()),
                    toml::to_string_pretty(&luro_user)?.as_bytes().to_vec(),
                    1
                )]);
            }

            if !luro_user.characters.is_empty() {
                writeln!(
                    user_data_description,
                    "- Has `{}` character profiles",
                    luro_user.characters.len()
                )?;
            }
            writeln!(user_data_description, "- Typed `{}` characters", luro_user.averagesize)?;
            writeln!(
                user_data_description,
                "- Has said `{}` words with an average length of `{}` characters per word",
                luro_user.wordcount,
                luro_user.averagesize.checked_div(luro_user.wordcount).unwrap_or(0)
            )?;

            if luro_user.moderation_actions_performed != 0 {
                writeln!(
                    user_data_description,
                    "- Performed `{}` moderation actions",
                    luro_user.moderation_actions_performed
                )?;
            }

            if luro_user.message_edits != 0 {
                writeln!(user_data_description, "- Edited `{}` messages", luro_user.message_edits)?;
            }

            if !luro_user.marriages.is_empty() {
                writeln!(user_data_description, "- Has `{}` marriages!", luro_user.marriages.len())?;
            }

            if !luro_user.moderation_actions.is_empty() {
                writeln!(
                    user_data_description,
                    "**Received `{}` punishments**",
                    luro_user.moderation_actions.len()
                )?;
                let (mut bans, mut kicks, mut warnings, mut priv_esc) = (0, 0, 0, 0);
                for punishment in &luro_user.moderation_actions {
                    for punishment_type in &punishment.action_type {
                        match punishment_type {
                            UserActionType::Ban => bans += 1,
                            UserActionType::Kick => kicks += 1,
                            UserActionType::Warn => warnings += 1,
                            UserActionType::PrivilegeEscalation => priv_esc += 1,
                            _ => ()
                        }
                    }
                }
                if bans != 0 {
                    writeln!(user_data_description, "- Banned `{bans}` times")?;
                }
                if kicks != 0 {
                    writeln!(user_data_description, "- Kicked `{kicks}` times")?;
                }
                if priv_esc != 0 {
                    writeln!(user_data_description, "- Attempted Privilege Escalation `{priv_esc}` times")?;
                }
                if warnings != 0 {
                    writeln!(user_data_description, "- Warned *(including expired)* `{warnings}` times")?;
                }
            }

            if !luro_user.warnings.is_empty() {
                writeln!(user_data_description, "- Has `{}` active warnings", luro_user.warnings.len())?;
            }

            match user_data_description.len() > 1024 {
                true => {
                    writeln!(description, "\n**User Data**\n{user_data_description}")?;
                }
                false => {
                    embed.create_field("User Data", &user_data_description, false);
                }
            }
        }

        if description.len() > 4096 {
            description.truncate(4093);
            description.push_str("...")
        }

        embed.create_field("Timestamps", &timestamp, true);
        embed.description(description);
        response.add_embed(embed);
        ctx.send_respond(response).await?;
        Ok(())
    }
}
