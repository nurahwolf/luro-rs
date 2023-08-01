use tracing::info;
use twilight_model::gateway::payload::incoming::ThreadMemberUpdate;

use crate::models::LuroFramework;

impl LuroFramework {
    pub async fn listener_thread_member_update(&self, event: Box<ThreadMemberUpdate>) -> anyhow::Result<()> {
        info!(thread = ?event, "Thread Member Update");
        // let guild_id = event.guild_id.unwrap_or_else(|| return Ok(()));
        // let guild_data = self.guild_data.get(&guild_id).unwrap_or_else(|| return Ok(()));
        // let log_channel = guild_data.discord_events_log_channel.unwrap_or_else(|| return Ok(()));
        // let embed = self.embed_thread_created(&event);

        // self.twilight_client
        //     .create_message(log_channel)
        //     .embeds(&[embed.build()])?
        //     .await?;

        Ok(())
    }
}
