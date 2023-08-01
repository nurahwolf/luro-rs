use async_trait::async_trait;
use std::{fmt::Write, time::Duration};
use tracing::debug;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::GenericMarker, Id};
use twilight_util::{
    builder::embed::{EmbedFieldBuilder, ImageSource},
    snowflake::Snowflake
};

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand)]
#[command(name = "user", desc = "Information about a user")]
pub struct UserCommands {
    /// The user to get, gets yourself if not specified
    user: Option<ResolvedUser>,
    /// Optionally try to get a user from a different guild
    guild: Option<Id<GenericMarker>>,
    /// Just show user details, not guild details
    user_only: Option<bool>
}

#[async_trait]
impl LuroCommand for UserCommands {
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
        let mut timestamp = format!("Joined discord on <t:{0}> - <t:{0}:R>\n", user_timestamp.as_secs());

        embed = embed.title(&user.name);
        embed = embed.thumbnail(ImageSource::url(self.get_user_avatar(&user))?);
        writeln!(description, "**ID:** `{0}` - <@{0}>", user.id)?;
        if let Some(accent_color) = user.accent_color {
            embed = embed.color(accent_color);
            writeln!(description, "**Accent Colour:** `{accent_color:X}`")?;
        }
        if let Some(email) = &user.email {
            writeln!(description, "**Email:** `{}`", email)?;
        }
        if let Some(locale) = &user.locale {
            writeln!(description, "**Locale:** `{}`", locale)?;
        }
        if let Some(mfa_enabled) = user.mfa_enabled {
            writeln!(description, "**MFA Enabled:** `{mfa_enabled}`")?;
        }
        if let Some(system) = user.system {
            writeln!(description, "**System Account:** `{system}`")?;
        }
        if let Some(verified) = user.verified {
            writeln!(description, "**Verified Account:** `{verified}`")?;
        }
        if user.bot {
            writeln!(description, "**Bot:** `true`")?;
        }
        if let Some(banner) = self.get_user_banner(&user) {
            embed = embed.image(ImageSource::url(&banner)?);
            writeln!(description, "**Banner:** {banner}")?;
        }

        // Some additional details if we are a guild
        let mut guild_id = ctx.interaction.guild_id;
        if let Some(requested_guild_id) = self.guild {
            guild_id = Some(Id::new(requested_guild_id.get()))
        };
        if let Some(guild_id) = guild_id && !self.user_only.is_some_and(|user_only| user_only) {
            let member = ctx.luro.twilight_client.guild_member(guild_id, user.id).await?.model().await?;
            embed = embed.thumbnail(ImageSource::url(self.get_member_avatar(Some(&member), &Some(guild_id), &user))?);
            writeln!(description, "\n-----\n**GUILD INFORMATION**")?;
            writeln!(description, "**Total Roles:** `{}`", member.roles.len())?;            

            if let Some(nickname) = member.nick {
                writeln!(description, "**Nickname:** `{nickname}`")?;            
            }
            if let Some(member_timestamp) = member.premium_since {
                timestamp.push_str(format!("Joined this server at <t:{0}> - <t:{0}:R>", member_timestamp.as_secs()).as_str());
            }
            if member.deaf {
                writeln!(description, "**Deafened:** `true`")?;            
            }
            if member.mute {
                writeln!(description, "**Muted:** `true`")?;            
            }
            if member.pending {
                writeln!(description, "**Pending:** `true`")?;            
            }
            if let Some(timestamp) = member.communication_disabled_until {
                writeln!(description, "**Timed out until:** <t:{}:R>", timestamp.as_secs())?;            
            }
            // TODO: Once member_banner is a thing in [Member]
            // if let Some(banner) = get_member_banner(&member, guild_id, user) {
            //     embed = embed.image(ImageSource::url(banner)?)
            // }       
        }

        embed = embed.field(EmbedFieldBuilder::new("Timestamps", timestamp));

        embed = embed.description(description);
        ctx.embed(embed.build())?.respond().await
    }
}
