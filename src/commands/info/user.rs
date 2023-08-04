use anyhow::Context;
use async_trait::async_trait;
use std::{fmt::Write, time::Duration};
use tracing::debug;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::GenericMarker, Id};
use twilight_util::{
    builder::embed::{EmbedFieldBuilder, ImageSource},
    snowflake::Snowflake
};

use crate::{
    models::{LuroSlash, RoleOrdering, UserActionType, UserData},
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
    user_only: Option<bool>
}

#[async_trait]
impl LuroCommand for InfoUser {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.deferred().await?;
        let mut embed = ctx.default_embed().await?;
        let mut description = String::new();
        // The user we are interested in is the interaction author, unless a user was specified
        let user = if let Some(ref user_specified) = self.user {
            match ctx.luro.twilight_cache.user(user_specified.resolved.id) {
                Some(user) => {
                    debug!("Using cached user");
                    user.clone()
                }
                None => {
                    debug!("Using client to fetch user");
                    ctx.luro
                        .twilight_client
                        .user(user_specified.resolved.id)
                        .await?
                        .model()
                        .await?
                }
            }
        } else {
            ctx.author()?
        };
        let user_timestamp = Duration::from_millis(user.id.timestamp().unsigned_abs());
        let mut timestamp = format!("- Joined discord on <t:{0}> - <t:{0}:R>\n", user_timestamp.as_secs());

        embed = embed.title(&user.name);
        embed = embed.thumbnail(ImageSource::url(ctx.user_get_avatar(&user))?);
        writeln!(description, "<@{0}> - `{0}`", user.id)?;
        if let Some(accent_color) = user.accent_color {
            embed = embed.color(accent_color);
            writeln!(description, "- Accent Colour: `{accent_color:X}`")?;
        }
        if let Some(email) = &user.email {
            writeln!(description, "- Email: `{}`", email)?;
        }
        if let Some(locale) = &user.locale {
            writeln!(description, "- Locale: `{}`", locale)?;
        }
        if let Some(mfa_enabled) = user.mfa_enabled {
            writeln!(description, "- MFA Enabled: `{mfa_enabled}`")?;
        }
        if let Some(system) = user.system {
            writeln!(description, "- System Account: `{system}`")?;
        }
        if let Some(verified) = user.verified {
            writeln!(description, " - Verified Account: `{verified}`")?;
        }
        if let Some(accent_color) = user.accent_color {
            writeln!(description, " - Accent Colour: `{accent_color:X}`")?;
        }
        if user.bot {
            writeln!(description, " - Bot: `true`")?;
        }
        if let Some(banner) = ctx.user_get_banner(&user) {
            embed = embed.image(ImageSource::url(&banner)?);
            writeln!(description, "- Banner: {banner}")?;
        }

        // Some additional details if we are a guild
        let mut guild_id = ctx.interaction.guild_id;
        if let Some(requested_guild_id) = self.guild {
            guild_id = Some(Id::new(requested_guild_id.get()))
        };
        if let Some(guild_id) = guild_id && !self.user_only.is_some_and(|user_only| user_only) {
            let mut guild_information = String::new();
            let member = ctx.luro.twilight_client.guild_member(guild_id, user.id).await?.model().await?;
            let guild = ctx.luro.twilight_client.guild(guild_id).await?.model().await?;

            embed = embed.thumbnail(ImageSource::url(ctx.member_get_avatar(Some(&member), &Some(guild_id), &user))?);

            let member = ctx.luro.twilight_cache.member(guild_id, user.id).context("Expected to find member in cache")?;
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
                    write!(role_list, "<@&{}>", role.id)?
                };
                write!(role_list, ", <@&{}>", role.id)?
            }
            writeln!(guild_information, "- Roles ({}): {role_list}", user_roles.len())?;

            timestamp.push_str(format!("- Joined this server at <t:{0}> - <t:{0}:R>", member.joined_at().as_secs()).as_str());       

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

        // USER DATA SECTION
        let mut user_data_description = String::new();
        let user_data = UserData::get_user_settings(&ctx.luro, &user.id).await?;

        writeln!(user_data_description, "- Total Words Said: `{}`", user_data.wordcount)?;
        writeln!(user_data_description, "- Total Characters Said: `{}`", user_data.averagesize)?;

        if user_data.moderation_actions_performed != 0 {
            writeln!(
                user_data_description,
                "- Moderation Actions Performed: `{}`",
                user_data.moderation_actions_performed
            )?;
        }

        if !user_data.moderation_actions.is_empty() {
            writeln!(
                user_data_description,
                "**Punishments Received - {}**",
                user_data.moderation_actions.len()
            )?;
            let (mut bans, mut kicks, mut warnings) = (0, 0, 0);
            for punishment in user_data.moderation_actions {
                for punishment_type in punishment.action_type {
                    match punishment_type {
                        UserActionType::Ban => bans += 1,
                        UserActionType::Kick => kicks += 1,
                        UserActionType::Warn => warnings += 1
                    }
                }
            }
            if bans != 0 {
                writeln!(user_data_description, "- Times Banned: `{bans}`")?;
            }
            if kicks != 0 {
                writeln!(user_data_description, "- Times Kicks: `{kicks}`")?;
            }
            if warnings != 0 {
                writeln!(user_data_description, "- Times Warned (including expired): `{warnings}`")?;
            }
        }

        if !user_data.warnings.is_empty() {
            writeln!(user_data_description, "- Warnings: `{}`", user_data.warnings.len())?;
        }

        match user_data_description.len() > 1024 {
            true => writeln!(description, "\n**User Data**\n{user_data_description}")?,
            false => embed = embed.field(EmbedFieldBuilder::new("Guild Information", user_data_description))
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
