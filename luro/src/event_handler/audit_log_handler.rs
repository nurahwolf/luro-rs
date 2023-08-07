use std::sync::Arc;

use tracing::warn;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::audit_log::AuditLogEventType};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFooterBuilder, ImageSource};

use crate::models::{LuroFramework, SlashUser};

mod member_ban_add;
mod member_ban_remove;
mod member_kick;

impl LuroFramework {
    pub async fn audit_log_handler(self: Arc<Self>, event: Box<GuildAuditLogEntryCreate>) -> anyhow::Result<()> {
        // Make sure this interaction was a guild
        let guild_id = match event.guild_id {
            Some(guild_id) => guild_id,
            None => {
                warn!("Expected a guild to be present when handling an audit log event");
                return Ok(());
            }
        };
        let mut embed = self.default_embed(&Some(guild_id));

        match event.user_id {
            Some(action_user_id) => {
                {
                    if action_user_id == self.global_data.read().current_user.id {
                        // Event done by the bot, so no need to report it again
                        return Ok(());
                    }
                }

                {
                    let (author, slash_author) = SlashUser::client_fetch_user(&self, action_user_id).await?;

                    let embed_author = EmbedAuthorBuilder::new(format!("Performed by {} - {}", slash_author.name, author.id))
                        .icon_url(ImageSource::url(slash_author.avatar)?)
                        .build();

                    embed = embed.author(embed_author)
                }
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
