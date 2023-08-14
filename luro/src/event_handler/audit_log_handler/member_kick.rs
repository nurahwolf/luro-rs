use crate::{framework::Framework, COLOUR_DANGER, functions::client_fetch};
use anyhow::Context;
use luro_builder::embed::EmbedBuilder;
use luro_model::{luro_database_driver::LuroDatabaseDriver, user_actions::UserActions, user_actions_type::UserActionType};
use std::{fmt::Write, sync::Arc};
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn subhandle_member_kick(
        self: &Arc<Self>,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let kicked_user_id = Id::new(event.target_id.context("No user ID found for kicked user")?.get());
        let slash_author = client_fetch(&self, Some(guild.id), kicked_user_id).await?;

        embed
            .thumbnail(|thumbnail| thumbnail.url(slash_author.avatar))
            .colour(COLOUR_DANGER)
            .title(format!("👢 Kicked from {}", guild.name));

        writeln!(
            description,
            "**User:** <@{kicked_user_id}> - `{}`\n**User ID:** `{kicked_user_id}`",
            slash_author.name
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
                self.database.modify_user(user_id, &reward).await?;

                // Record the punishment
                let mut banned = self.database.get_user(&kicked_user_id).await?;
                banned.moderation_actions.push(UserActions {
                    action_type: vec![UserActionType::Kick],
                    guild_id: Some(guild.id),
                    reason: reason.clone(),
                    responsible_user: kicked_user_id
                });
                self.database.modify_user(&kicked_user_id, &banned).await?;
            }
        }
        embed.description(description);
        self.send_moderator_log_channel(&Some(guild.id), embed).await
    }
}
