use luro_model::{database::drivers::LuroDatabaseDriver, user::LuroUser};
use std::sync::Arc;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild};

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn subhandle_member_ban_remove(
        self: &Arc<Self>,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate,
        moderator: &mut LuroUser,
        punished_user: &LuroUser
    ) -> anyhow::Result<()> {
        let embed = self.kick_embed(&guild.name, &guild.id, moderator, punished_user, event.reason.as_deref());

        // Reward the moderator
        moderator.moderation_actions_performed += 1;
        self.database.save_user(&moderator.id(), moderator).await?;

        // Send the response
        self.send_moderator_log_channel(&Some(guild.id), embed).await
    }
}
