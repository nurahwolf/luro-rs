use luro_framework::responses::{PunishmentType, StandardResponse};
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    user::{actions::UserActions, actions_type::UserActionType, LuroUser}
};
use std::sync::Arc;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild};

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn subhandle_member_ban_add(
        self: &Arc<Self>,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate,
        moderator: &mut LuroUser,
        punished_user: &mut LuroUser
    ) -> anyhow::Result<()> {
        let mut response =
            StandardResponse::new_punishment(PunishmentType::Banned, &guild.name, &guild.id, punished_user, moderator);
        response.punishment_reason(event.reason.as_deref(), punished_user);

        // Reward the moderator
        moderator.moderation_actions_performed += 1;
        self.update_user(moderator).await?;
        self.database.save_user(&moderator.id, moderator).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Ban],
            guild_id: Some(guild.id),
            reason: event.reason.clone(),
            responsible_user: moderator.id
        });
        self.update_user(punished_user).await?;
        self.database.save_user(&punished_user.id, punished_user).await?;

        // Send the response
        self.send_moderator_log_channel(&Some(guild.id), response.embed).await
    }
}
