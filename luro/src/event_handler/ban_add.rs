use std::sync::Arc;

use tracing::info;
use twilight_model::gateway::payload::incoming::BanAdd;
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{framework::LuroFramework, functions::base_embed};

impl LuroFramework {
    pub async fn ban_add_listener(self: &Arc<Self>, ban: BanAdd) -> anyhow::Result<()> {
        info!("User {} banned from guild {}", ban.user.name, ban.guild_id);

        // Exit early if it was the bot that performed the ban
        if ban.user.id == self.twilight_client.current_user().await?.model().await?.id {
            return Ok(());
        }

        let guild_db = self.guilds.read().clone();
        let guild_settings = match guild_db.get(&ban.guild_id) {
            Some(guild_settings) => guild_settings,
            None => return Ok(()),
        };

        if let Some(moderator_actions_log_channel) = guild_settings.moderator_actions_log_channel {
            let (banned_user_id, banned_user_name) = (ban.user.id, ban.user.name);
            let resolved_ban = self
                .twilight_client
                .ban(ban.guild_id, ban.user.id)
                .await?
                .model()
                .await?;
            let guild = self
                .twilight_client
                .guild(ban.guild_id)
                .await?
                .model()
                .await?;

            let mut embed = base_embed(self, Some(ban.guild_id))
                .await
                .title(format!("Banned from {}", guild.name))
                .field(EmbedFieldBuilder::new("Guild ID", guild.id.to_string()).inline());

                match resolved_ban.reason {
                    Some(reason) =>                     embed = embed.description(format!(
                        "**User:** <@{banned_user_id}> - {banned_user_name}\n**User ID:** {banned_user_id}\n**Reason:** ```{reason}```",
                    )),
                    None =>                     embed = embed.description(format!(
                        "**User:** <@{banned_user_id}> - {banned_user_name}\n**User ID:** {banned_user_id}",
                    )),
                };

            self.twilight_client
                .create_message(moderator_actions_log_channel)
                .embeds(&[embed.build()])?;
        }

        Ok(())
    }
}
