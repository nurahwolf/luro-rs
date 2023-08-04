use crate::{models::UserData, traits::toml::LuroTOML, USERDATA_FILE_PATH};
use anyhow::Context;
use std::{fmt::Write, path::Path, sync::Arc};
use twilight_model::{gateway::payload::incoming::GuildAuditLogEntryCreate, guild::Guild, id::Id};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

use crate::{
    models::{LuroFramework, UserActionType, UserActions},
    traits::luro_functions::LuroFunctions,
    COLOUR_DANGER
};

impl LuroFramework {
    pub async fn subhandle_member_ban_add(
        self: &Arc<Self>,
        mut embed: EmbedBuilder,
        guild: &Guild,
        event: &GuildAuditLogEntryCreate
    ) -> anyhow::Result<()> {
        let mut description = String::new();
        let banned_user_id = Id::new(event.target_id.context("No user ID found for banned user")?.get());
        let (_user, avatar, name) = self.fetch_specified_user(self, &banned_user_id).await?;

        embed = embed
            .thumbnail(ImageSource::url(avatar)?)
            .color(COLOUR_DANGER)
            .title(format!("🔨 Banned from {}", guild.name));

        writeln!(
            description,
            "**User:** <@{banned_user_id}> - `{name}`\n**User ID:** `{banned_user_id}`"
        )?;

        if let Some(reason) = &event.reason {
            if reason.starts_with("```") {
                writeln!(description, "{reason}")?
            } else {
                writeln!(description, "```{reason}```")?
            }
            if let Some(user_id) = &event.user_id {
                let _ = UserData::get_user_settings(self, user_id).await?;
                let _ = UserData::get_user_settings(self, &banned_user_id).await?;
                // Reward the person who actioned the ban
                let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &user_id);
                let data = &mut self
                    .user_data
                    .get_mut(user_id)
                    .context("Expected to find user's data in the cache")?;
                data.moderation_actions_performed += 1;
                data.write(Path::new(&path)).await?;
                // Record the punishment
                let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &banned_user_id);
                let data = &mut self
                    .user_data
                    .get_mut(user_id)
                    .context("Expected to find user's data in the cache")?;
                data.moderation_actions.push(UserActions {
                    action_type: vec![UserActionType::Ban],
                    guild_id: Some(event.guild_id.context("Expected this to be a guild")?),
                    reason: reason.clone(),
                    responsible_user: *user_id
                });
                data.write(Path::new(&path)).await?;
            }
        }
        embed = embed.description(description);

        self.send_moderator_log_channel(&Some(guild.id), embed).await
    }
}
