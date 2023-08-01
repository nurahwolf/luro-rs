use anyhow::Context;
use std::fmt::Write;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

use crate::{functions::get_user_avatar, models::LuroFramework, COLOUR_DANGER};

impl LuroFramework {
    pub async fn subhandle_member_ban_add(
        &self,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let banned_user_id = Id::new(event.target_id.context("No user ID found for banned user")?.get());

        let resolved_ban = self.twilight_client.ban(guild.id, banned_user_id).await?.model().await?;
        let banned_user_name = if resolved_ban.user.discriminator == 0 {
            resolved_ban.user.name.clone()
        } else {
            format!("{}#{}", resolved_ban.user.name, resolved_ban.user.discriminator)
        };
        let banned_avatar = get_user_avatar(&resolved_ban.user);
        embed = embed
            .thumbnail(ImageSource::url(banned_avatar)?)
            .color(COLOUR_DANGER)
            .title(format!("🔨 Banned from {}", guild.name));

        writeln!(
            description,
            "**User:** <@{banned_user_id}> - `{banned_user_name}`\n**User ID:** `{banned_user_id}`"
        )?;

        if let Some(reason) = &event.reason {
            if reason.starts_with("```") {
                writeln!(description, "{reason}")?
            } else {
                writeln!(description, "```{reason}```")?
            }
        }
        embed = embed.description(description);
        self.send_moderator_log_channel(&Some(guild.id), embed).await
    }
}
