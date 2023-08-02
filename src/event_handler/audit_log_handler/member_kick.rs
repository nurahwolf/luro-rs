use anyhow::Context;
use std::fmt::Write;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

use crate::{models::LuroFramework, traits::luro_functions::LuroFunctions, COLOUR_DANGER};

impl LuroFramework {
    pub async fn subhandle_member_kick(
        &self,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let kicked_user_id = Id::new(event.target_id.context("No user ID found for kicked user")?.get());
        let (_author, avatar, name) = self.fetch_specified_user(&self, &kicked_user_id).await?;
        embed = embed
            .thumbnail(ImageSource::url(avatar)?)
            .color(COLOUR_DANGER)
            .title(format!("👢 Kicked from {}", guild.name));

        writeln!(
            description,
            "**User:** <@{kicked_user_id}> - `{name}`\n**User ID:** `{kicked_user_id}`"
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
