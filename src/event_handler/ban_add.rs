use std::sync::Arc;

use tracing::info;
use twilight_model::gateway::payload::incoming::BanAdd;
use twilight_util::builder::embed::{EmbedFieldBuilder, ImageSource};

use crate::{
    functions::{default_embed, get_user_avatar},
    models::LuroFramework
};

impl LuroFramework {
    pub async fn ban_add_listener(self: Arc<Self>, ban: BanAdd) -> anyhow::Result<()> {
        info!("User {} banned from guild {}", ban.user.name, ban.guild_id);
        let guild_db = self.guild_data.read().clone();
        let guild_settings = match guild_db.get(&ban.guild_id) {
            Some(guild_settings) => guild_settings,
            None => return Ok(())
        };

        if let Some(moderator_actions_log_channel) = guild_settings.moderator_actions_log_channel {
            let (banned_user_id, banned_user_name) = (ban.user.id, ban.user.name);
            let resolved_ban = self.twilight_client.ban(ban.guild_id, ban.user.id).await?.model().await?;
            let guild = self.twilight_client.guild(ban.guild_id).await?.model().await?;
            let banned_avatar = get_user_avatar(&resolved_ban.user);

            let mut embed = default_embed(&self, &Some(ban.guild_id))
                .title(format!("Banned from {}", guild.name))
                .thumbnail(ImageSource::url(banned_avatar)?);

            embed = embed.field(EmbedFieldBuilder::new("User", format!("<@{banned_user_id}> - {banned_user_name}>")).inline());
            embed = embed.field(EmbedFieldBuilder::new("User ID", banned_user_id.to_string()).inline());
            embed = embed.field(EmbedFieldBuilder::new("Guild ID", guild.id.to_string()).inline());

            if let Some(reason) = resolved_ban.reason {
                embed = embed.field(EmbedFieldBuilder::new("User", format!("```{reason}```")));
            }

            self.twilight_client
                .create_message(moderator_actions_log_channel)
                .embeds(&[embed.build()])?
                .await?;
        }

        Ok(())
    }
}
