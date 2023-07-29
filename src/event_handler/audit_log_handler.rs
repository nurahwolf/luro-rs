use anyhow::Context;
use std::fmt::Write;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::audit_log::AuditLogEventType, id::Id};
use twilight_util::builder::embed::{EmbedAuthorBuilder, ImageSource};

use crate::{
    functions::{default_embed, get_user_avatar},
    models::LuroFramework,
    COLOUR_DANGER, COLOUR_SUCCESS
};

impl LuroFramework {
    pub async fn audit_log_handler(&self, event: Box<GuildAuditLogEntryCreate>) -> anyhow::Result<()> {
        let moderation_actions_log_channel;
        let guild_id = event.guild_id.context("No guild id in this event")?;

        {
            let guild_db = self.guild_data.read();
            let guild_settings = guild_db.get(&guild_id).context("No guild settings available")?;

            match guild_settings.moderator_actions_log_channel {
                Some(settings) => moderation_actions_log_channel = settings,
                None => return Ok(())
            }
        }

        if let Some(action_user_id) = event.user_id {
            if action_user_id == self.global_data.read().current_user.id {
                // Event done by the bot, so no need to report it again
                return Ok(());
            }

            let mut description = String::new();
            let author = self.twilight_client.user(action_user_id).await?.model().await?;
            let author_avatar = get_user_avatar(&author);
            let author_name = if author.discriminator == 0 {
                author.name
            } else {
                format!("{}#{}", author.name, author.discriminator)
            };

            let guild = self.twilight_client.guild(guild_id).await?.model().await?;

            let embed_author = EmbedAuthorBuilder::new(format!("Performed by {} - {}", author_name, author.id))
                .icon_url(ImageSource::url(author_avatar)?)
                .build();
            let mut embed = default_embed(self, &Some(guild_id)).author(embed_author);

            // TODO: Move this to it's own function
            match event.action_type {
                AuditLogEventType::MemberBanAdd => {
                    let banned_user_id = Id::new(event.target_id.context("No user ID found for banned user")?.get());

                    let resolved_ban = self.twilight_client.ban(guild_id, banned_user_id).await?.model().await?;
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
                }
                AuditLogEventType::MemberKick => {
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
                }
                AuditLogEventType::MemberBanRemove => {
                    let unbanned_user_id = Id::new(event.target_id.context("No user ID found for unbanned user")?.get());
                    let unbanned_user = self.twilight_client.user(unbanned_user_id).await?.model().await?;
                    let unbanned_user_name = if unbanned_user.discriminator == 0 {
                        unbanned_user.name.clone()
                    } else {
                        format!("{}#{}", unbanned_user.name, unbanned_user.discriminator)
                    };
                    let unbanned_user_avatar: String = get_user_avatar(&unbanned_user);
                    embed = embed
                        .thumbnail(ImageSource::url(unbanned_user_avatar)?)
                        .color(COLOUR_SUCCESS)
                        .title(format!("🔓 Unbanned from {}", guild.name));
                    writeln!(
                        description,
                        "**User:** <@{unbanned_user_id}> - `{unbanned_user_name}`\n**User ID:** `{unbanned_user_id}`"
                    )?;
                }
                _ => return Ok(())
            }

            if let Some(reason) = &event.reason {
                if reason.starts_with("```") {
                    writeln!(description, "{reason}")?
                } else {
                    writeln!(description, "```{reason}```")?
                }
            }

            embed = embed.description(description);

            self.twilight_client
                .create_message(moderation_actions_log_channel)
                .embeds(&[embed.build()])?
                .await?;
        }

        Ok(())
    }
}
