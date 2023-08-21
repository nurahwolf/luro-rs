use anyhow::Context;
use luro_builder::embed::EmbedBuilder;
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    user::{actions::UserActions, actions_type::UserActionType}
};
use std::fmt::Write;
use std::sync::Arc;
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};

use crate::{framework::Framework, COLOUR_DANGER};

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn subhandle_member_ban_add(
        self: &Arc<Self>,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let banned_user_id = Id::new(event.target_id.context("No user ID found for banned user")?.get());
        let luro_user = self.database.get_user(&banned_user_id).await?;

        embed
            .thumbnail(|thumbnail| thumbnail.url(luro_user.avatar()))
            .title(format!("🔨 Banned from {}", guild.name))
            .colour(COLOUR_DANGER);

        writeln!(
            description,
            "**User:** <@{banned_user_id}> - `{}`\n**User ID:** `{banned_user_id}`",
            luro_user.name()
        )?;

        if let Some(reason) = &event.reason {
            if reason.starts_with("```") {
                writeln!(description, "{reason}")?
            } else {
                writeln!(description, "```{reason}```")?
            }
            if let Some(user_id) = &event.user_id {
                let mut reward = self.database.get_user(user_id).await?;
                reward.moderation_actions_performed += 1;
                self.database.save_user(user_id, &reward).await?;

                // Record the punishment
                let mut banned = self.database.get_user(&banned_user_id).await?;
                banned.moderation_actions.push(UserActions {
                    action_type: vec![UserActionType::Ban],
                    guild_id: Some(guild.id),
                    reason: reason.clone(),
                    responsible_user: banned_user_id
                });
                self.database.save_user(&banned_user_id, &banned).await?;
            }
        }
        embed.description(description);

        self.send_moderator_log_channel(&Some(guild.id), embed).await
    }
}
