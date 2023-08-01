use anyhow::Context;
use std::fmt::Write;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

use crate::{functions::get_user_avatar, models::LuroFramework, COLOUR_DANGER};

impl LuroFramework {
    pub async fn subhandle_member_kick(
        &self,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let kicked_user_id = Id::new(event.target_id.context("No user ID found for kicked user")?.get());
        let kicked_user = self.twilight_client.user(kicked_user_id).await?.model().await?;
        let kicked_user_name = if kicked_user.discriminator == 0 {
            kicked_user.name.clone()
        } else {
            format!("{}#{}", kicked_user.name, kicked_user.discriminator)
        };
        let kicked_user_avatar: String = get_user_avatar(&kicked_user);
        embed = embed
            .thumbnail(ImageSource::url(kicked_user_avatar)?)
            .color(COLOUR_DANGER)
            .title(format!("👢 Kicked from {}", guild.name));

        writeln!(
            description,
            "**User:** <@{kicked_user_id}> - `{kicked_user_name}`\n**User ID:** `{kicked_user_id}`"
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
