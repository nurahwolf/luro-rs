use std::sync::Arc;

use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::audit_log::AuditLogEventType};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFooterBuilder, ImageSource};

use crate::{functions::get_user_avatar, models::LuroFramework};

mod member_ban_add;
mod member_ban_remove;
mod member_kick;

impl LuroFramework {
    pub async fn audit_log_handler(self: Arc<Self>, event: Box<GuildAuditLogEntryCreate>) -> anyhow::Result<()> {
        // Make sure this interaction was a guild
        let guild_id = match event.guild_id {
            Some(guild_id) => guild_id,
            None => return Ok(())
        };
        let mut embed = self.default_embed(&Some(guild_id));

        match event.user_id {
            Some(action_user_id) => {
                if action_user_id == self.global_data.read().current_user.id {
                    // Event done by the bot, so no need to report it again
                    return Ok(());
                }

                let author = self.twilight_client.user(action_user_id).await?.model().await?;
                let author_avatar = get_user_avatar(&author);
                let author_name = if author.discriminator == 0 {
                    author.name
                } else {
                    format!("{}#{}", author.name, author.discriminator)
                };

                let embed_author = EmbedAuthorBuilder::new(format!("Performed by {} - {}", author_name, author.id))
                    .icon_url(ImageSource::url(author_avatar)?)
                    .build();

                embed = embed.author(embed_author)
            }
            None => embed = embed.footer(EmbedFooterBuilder::new("There is no record who performed this action."))
        };

        let guild = self.twilight_client.guild(guild_id).await?.model().await?;
        match event.action_type {
            AuditLogEventType::MemberBanAdd => self.subhandle_member_ban_add(embed, &guild, &event).await,
            AuditLogEventType::MemberKick => self.subhandle_member_kick(embed, &guild, &event).await,
            AuditLogEventType::MemberBanRemove => self.subhandle_member_ban_remove(embed, &guild, &event).await,
            _ => Ok(())
        }
    }
}
