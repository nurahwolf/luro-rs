use std::sync::Arc;

use crate::framework::Framework;
use luro_framework::responses::{PunishmentType, StandardResponse};
use luro_model::{
    database_driver::LuroDatabaseDriver,
    user::{actions::UserActions, actions_type::UserActionType, LuroUser},
};
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild};

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn subhandle_member_kick(
        self: &Arc<Self>,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate,
        moderator: &mut LuroUser,
        punished_user: &mut LuroUser,
    ) -> anyhow::Result<()> {
        let mut response =
            StandardResponse::new_punishment(PunishmentType::Kicked, &guild.name, &guild.id, punished_user, moderator);
        response.punishment_reason(event.reason.as_deref(), punished_user);
        // Reward the moderator
        moderator.moderation_actions_performed += 1;
        self.database.modify_user(&moderator.id, moderator).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Kick],
            guild_id: Some(guild.id),
            reason: event.reason.clone(),
            responsible_user: moderator.id,
        });
        self.database.modify_user(&punished_user.id, punished_user).await?;

        // Send the response
        self.send_moderator_log_channel(&Some(guild.id), response.embed).await
    }
}
