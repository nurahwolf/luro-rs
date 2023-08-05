use anyhow::Context;
use async_trait::async_trait;
use std::{convert::TryInto, fmt::Write, time::Duration};

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{id::{marker::GenericMarker, Id}, http::attachment::Attachment};
use twilight_util::{
    builder::embed::{EmbedAuthorBuilder, EmbedFieldBuilder, ImageSource},
    snowflake::Snowflake
};

use crate::{
    models::{LuroSlash, RoleOrdering, SlashUser, UserActionType, UserData},
    traits::luro_functions::LuroFunctions
};

use crate::traits::luro_command::LuroCommand;

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
    gdpr_export: Option<bool>,
}

#[async_trait]
impl LuroCommand for InfoUser {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.deferred().await?;
        let mut embed = ctx.default_embed().await?;
        let mut description = String::new();
        // The user we are interested in is the interaction author, unless a user was specified
        let (author, slash_author) = ctx.get_specified_user_or_author(&self.user, &ctx.interaction)?;
        let user_timestamp = Duration::from_millis(author.id.timestamp().unsigned_abs());
        let mut timestamp = format!("- Joined discord on <t:{0}> - <t:{0}:R>\n", user_timestamp.as_secs());

        embed = embed.author(EmbedAuthorBuilder::new(&slash_author.name).icon_url(slash_author.clone().try_into()?));
        if let Some(hide_avatar) = self.hide_avatar && hide_avatar {
        } else {
            embed = embed.thumbnail(slash_author.clone().try_into()?);
        }

        writeln!(description, "<@{0}> - `{0}`", author.id)?;
        if let Some(accent_color) = author.accent_color {
            embed = embed.color(accent_color);
            writeln!(description, "- Accent Colour: `{accent_color:X}`")?;
        }
        if let Some(email) = &author.email {
            writeln!(description, "- Email: `{}`", email)?;
        }
        if let Some(locale) = &author.locale {
            writeln!(description, "- Locale: `{}`", locale)?;
        }
        if let Some(mfa_enabled) = author.mfa_enabled {
            writeln!(description, "- MFA Enabled: `{mfa_enabled}`")?;
        }
        if let Some(system) = author.system {
            writeln!(description, "- System Account: `{system}`")?;
        }
        if let Some(verified) = author.verified {
            writeln!(description, " - Verified Account: `{verified}`")?;
        }
        if author.bot {
            writeln!(description, " - Bot: `true`")?;
        }
        if let Some(ref banner) = slash_author.banner {
            embed = embed.image(ImageSource::url(banner)?);
        }

        // Some additional details if we are a guild
        let guild_id = match self.guild {
            Some(guild_specified) => Some(Id::new(guild_specified.get())),
            None => ctx.interaction.guild_id
        };

        if let Some(guild_id) = guild_id && !self.user_only.is_some_and(|user_only| user_only) {
            if let Ok(member) = ctx.luro.twilight_client.guild_member(guild_id, author.id).await {
                    let member = member.model().await?;
                    let slash_member = SlashUser::from_member(&member.user, member.avatar, Some(guild_id));
                    let mut guild_information = String::new();
                    let guild = ctx.luro.twilight_client.guild(guild_id).await?.model().await?;
                    embed = embed.author(EmbedAuthorBuilder::new(slash_member.name).icon_url(slash_author.clone().try_into()?));
                    if let Some(hide_avatar) = self.hide_avatar && hide_avatar {
                    } else {
                        embed = embed.thumbnail(slash_author.clone().try_into()?);
                    }
                    let member = ctx.luro.twilight_cache.member(guild_id, author.id).context("Expected to find member in cache")?;
                    let mut user_roles = vec![];
                    for member_role in member.roles() {
                        for guild_role in guild.roles.clone() {
                            if member_role == &guild_role.id {
                                user_roles.push(guild_role)
                            }
                        }
                    }
                    let mut user_roles: Vec<_> = user_roles.iter().map(RoleOrdering::from).collect();
                    user_roles.sort_by(|a, b| b.cmp(a));
                    let mut role_list = String::new();

                    for role in &user_roles {
                        if role_list.is_empty() {
                            write!(role_list, "<@&{}>", role.id)?;
                            continue;
                        };
                        write!(role_list, ", <@&{}>", role.id)?
                    }

                    if let Some(role) = user_roles.first() {
                        if role.colour != 0 {
                            embed = embed.color(role.colour);
                        }
                    }
                    writeln!(guild_information, "- Roles ({}): {role_list}", user_roles.len())?;
                    timestamp.push_str(format!("- Joined this server at <t:{0}> - <t:{0}:R>\n", member.joined_at().as_secs()).as_str());
                    if let Some(member_timestamp) = member.premium_since() {
                        timestamp.push_str(format!("- Boosted this server since <t:{0}> - <t:{0}:R>", member_timestamp.as_secs()).as_str());
                    }
                    if let Some(nickname) = member.nick() {
                        writeln!(guild_information, "**- Nickname:** `{nickname}`")?;            
                    }
                    if let Some(deaf) = member.deaf() && deaf {
                        writeln!(guild_information, "**- Deafened:** `true`")?;            
                    }
                    if let Some(mute) = member.mute() && mute {
                        writeln!(guild_information, "**- Muted:** `true`")?;            
                    }
                    if member.pending() {
                        writeln!(guild_information, "**- Pending:** `true`")?;            
                    }
                    if let Some(timestamp) = member.communication_disabled_until() {
                        writeln!(guild_information, "**- Timed out until:** <t:{}:R>", timestamp.as_secs())?;            
                    }
                    // TODO: Once member_banner is a thing in [Member]
                    // if let Some(banner) = get_member_banner(&member, guild_id, user) {
                    //     embed = embed.image(ImageSource::url(banner)?)
                    // }
                    match guild_information.len() > 1024 {
                        true => writeln!(description, "\n**Guild Information**\n{guild_information}")?,
                        false => embed = embed.field(EmbedFieldBuilder::new("Guild Information", guild_information))
                    }
            }
        }

        // USER DATA SECTION
        let mut user_data_description = String::new();
        {
            let user_data = UserData::get_user_settings(&ctx.luro, &author.id).await?;

            writeln!(user_data_description, "- Total Words Said: `{}`", user_data.wordcount)?;
            writeln!(user_data_description, "- Total Characters Said: `{}`", user_data.averagesize)?;

            if user_data.moderation_actions_performed != 0 {
                writeln!(
                    user_data_description,
                    "- Performed `{}` moderation actions",
                    user_data.moderation_actions_performed
                )?;
            }

            if user_data.message_edits != 0 {
                writeln!(user_data_description, "- Edited `{}` messages", user_data.message_edits)?;
            }

            if !user_data.moderation_actions.is_empty() {
                writeln!(
                    user_data_description,
                    "**Received `{}` punishments**",
                    user_data.moderation_actions.len()
                )?;
                let (mut bans, mut kicks, mut warnings, mut priv_esc) = (0, 0, 0, 0);
                for punishment in &user_data.moderation_actions {
                    for punishment_type in &punishment.action_type {
                        match punishment_type {
                            UserActionType::Ban => bans += 1,
                            UserActionType::Kick => kicks += 1,
                            UserActionType::Warn => warnings += 1,
                            UserActionType::PrivilegeEscalation => priv_esc += 1
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
                    writeln!(user_data_description, "- Attempts Privilege Escalation `{priv_esc}` times")?;
                }
                if warnings != 0 {
                    writeln!(user_data_description, "- Warned *(including expired)* `{warnings}` times")?;
                }
            }

            if !user_data.warnings.is_empty() {
                writeln!(user_data_description, "- Has `{}` active warnings", user_data.warnings.len())?;
            }

            match user_data_description.len() > 1024 {
                true => writeln!(description, "\n**User Data**\n{user_data_description}")?,
                false => embed = embed.field(EmbedFieldBuilder::new("User Data", user_data_description))
            }

            if let Some(export) = self.gdpr_export && export {
                if let Some(user_specified) = self.user {
                    // TODO: Add privilege esc tally to the person
                    return ctx.content(format!("Hey <@{}>! <@{}> is being a cunt and trying to steal your data.", ctx.author()?.id, user_specified.resolved.id)).respond().await
                }
    
                ctx.attachments = Some(vec![Attachment::from_bytes(
                    format!("gdpr-export-{}.txt", ctx.author()?.id),
                    toml::to_string_pretty(&user_data)?.as_bytes().to_vec(),
                    1
                )]);
            }
        }

        if description.len() > 4096 {
            description.truncate(4093);
            description.push_str("...")
        }



        embed = embed.field(EmbedFieldBuilder::new("Timestamps", timestamp).inline());
        embed = embed.description(description);
        ctx.embed(embed.build())?.respond().await
    }
}
