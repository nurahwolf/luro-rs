use anyhow::Context;
use std::fmt::Write;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

use crate::{models::LuroFramework, traits::luro_functions::LuroFunctions, COLOUR_DANGER};

impl LuroFramework {
    pub async fn subhandle_member_ban_add(
        &self,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let banned_user_id = Id::new(event.target_id.context("No user ID found for banned user")?.get());
        let (_user, avatar, name) = self.fetch_specified_user(&self, &banned_user_id).await?;
        let _resolved_ban = self.twilight_client.ban(guild.id, banned_user_id).await?.model().await?;

        embed = embed
            .thumbnail(ImageSource::url(avatar)?)
            .color(COLOUR_DANGER)
            .title(format!("🔨 Banned from {}", guild.name));

        writeln!(
            description,
            "**User:** <@{banned_user_id}> - `{name}`\n**User ID:** `{banned_user_id}`"
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
