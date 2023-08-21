use anyhow::Context;
use luro_builder::embed::EmbedBuilder;
use luro_model::database::drivers::LuroDatabaseDriver;
use std::{fmt::Write, sync::Arc};
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};

use crate::{framework::Framework, COLOUR_SUCCESS};

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn subhandle_member_ban_remove(
        self: &Arc<Self>,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let unbanned_user_id = Id::new(event.target_id.context("No user ID found for unbanned user")?.get());
        let luro_user = self.database.get_user(&unbanned_user_id).await?;

        embed
            .thumbnail(|thumbnail| thumbnail.url(luro_user.avatar()))
            .colour(COLOUR_SUCCESS)
            .title(format!("🔓 Unbanned from {}", guild.name));
        writeln!(
            description,
            "**User:** <@{unbanned_user_id}> - `{}`\n**User ID:** `{unbanned_user_id}`",
            luro_user.name()
        )?;

        if let Some(reason) = &event.reason {
            if reason.starts_with("```") {
                writeln!(description, "{reason}")?
            } else {
                writeln!(description, "```{reason}```")?
            }
        }
        embed.description(description);
        self.send_moderator_log_channel(&Some(guild.id), embed).await
    }
}
