use std::sync::Arc;

use anyhow::Context;
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::warn;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::audit_log::AuditLogEventType};

use crate::framework::Framework;

mod member_ban_add;
mod member_ban_remove;
mod member_kick;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn audit_log_handler(self: Arc<Self>, event: Box<GuildAuditLogEntryCreate>) -> anyhow::Result<()> {
        let punished_user_id = &event.target_id.context("No user ID for the punished user")?.cast();
        let mut punished_user = self.database.get_user(punished_user_id, &self.twilight_client).await?;
        let mut moderator = self
            .database
            .get_user(
                &event.user_id.context("No user ID for the ban author")?,
                &self.twilight_client
            )
            .await?;
        // Make sure this interaction was a guild
        let guild_id = match event.guild_id {
            Some(guild_id) => guild_id,
            None => {
                warn!("Expected a guild to be present when handling an audit log event");
                return Ok(());
            }
        };
        let guild = self.twilight_client.guild(guild_id).await?.model().await?;

        if moderator.id() == self.database.current_user.read().unwrap().id {
            // Event done by the bot, so no need to report it again
            return Ok(());
        }

        match event.action_type {
            AuditLogEventType::MemberBanAdd => {
                self.subhandle_member_ban_add(&guild, &event, &mut moderator, &mut punished_user)
                    .await
            }
            AuditLogEventType::MemberKick => {
                self.subhandle_member_kick(&guild, &event, &mut moderator, &mut punished_user)
                    .await
            }
            AuditLogEventType::MemberBanRemove => {
                self.subhandle_member_ban_remove(&guild, &event, &mut moderator, &punished_user)
                    .await
            }
            _ => {
                let mut guild_settings = self.database.get_guild(&guild_id).await?;
                guild_settings.update_guild(guild);
                self.database.save_guild(&guild_id, &guild_settings).await?;
                Ok(())
            }
        }
    }
}
