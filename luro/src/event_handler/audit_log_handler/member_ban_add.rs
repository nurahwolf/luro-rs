use crate::{framework::Framework, models::SlashUser};
use luro_model::{luro_database_driver::LuroDatabaseDriver, user_actions::UserActions, user_actions_type::UserActionType};

use anyhow::Context;
use std::{convert::TryInto, fmt::Write, sync::Arc};
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn subhandle_member_ban_add(
        self: &Arc<Self>,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let banned_user_id = Id::new(event.target_id.context("No user ID found for banned user")?.get());
        let (_, slash_author) = SlashUser::client_fetch_user(self, banned_user_id).await?;

        embed = embed
            .thumbnail(slash_author.clone().try_into()?)
            .color(COLOUR_DANGER)
            .title(format!("🔨 Banned from {}", guild.name));

        writeln!(
            description,
            "**User:** <@{banned_user_id}> - `{}`\n**User ID:** `{banned_user_id}`",
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
                let mut banned = self.database.get_user(&banned_user_id).await?;
                banned.moderation_actions.push(UserActions {
                    action_type: vec![UserActionType::Ban],
                    guild_id: Some(guild.id),
                    reason: reason.clone(),
                    responsible_user: banned_user_id
                });
                self.database.modify_user(&banned_user_id, &banned).await?;
            }
        }
        embed = embed.description(description);

        self.send_moderator_log_channel(&Some(guild.id), embed).await
    }
}
