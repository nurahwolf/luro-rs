use anyhow::Context;
use std::{fmt::Write, sync::Arc};
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

use crate::{
    framework::LuroFramework,
    models::{LuroLogChannel, SlashUser},
    COLOUR_SUCCESS
};

impl LuroFramework {
    pub async fn subhandle_member_ban_remove(
        self: &Arc<Self>,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let unbanned_user_id = Id::new(event.target_id.context("No user ID found for unbanned user")?.get());
        let (_, slash_author) = SlashUser::client_fetch_user(self, unbanned_user_id).await?;

        embed = embed
            .thumbnail(ImageSource::url(slash_author.avatar)?)
            .color(COLOUR_SUCCESS)
            .title(format!("🔓 Unbanned from {}", guild.name));
        writeln!(
            description,
            "**User:** <@{unbanned_user_id}> - `{}`\n**User ID:** `{unbanned_user_id}`",
            slash_author.name
        )?;

        if let Some(reason) = &event.reason {
            if reason.starts_with("```") {
                writeln!(description, "{reason}")?
            } else {
                writeln!(description, "```{reason}```")?
            }
        }
        embed = embed.description(description);
        self.send_log_channel(&Some(guild.id), embed, LuroLogChannel::Moderator).await
    }
}
