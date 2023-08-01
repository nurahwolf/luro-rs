use tracing::info;
use twilight_model::gateway::payload::incoming::ThreadListSync;

use crate::models::LuroFramework;

impl LuroFramework {
    pub async fn listener_thread_list_sync(&self, event: ThreadListSync) -> anyhow::Result<()> {
        info!(thread = ?event, "Thread created");
        let guild_id = event.guild_id;
        let guild_data = match self.guild_data.get(&guild_id) {
            Some(data) => data,
            None => return Ok(())
        };
        let _log_channel = match guild_data.discord_events_log_channel {
            Some(data) => data,
            None => return Ok(())
        };
        // let embed = self.embed_thread_created(&event);

        // self.twilight_client
        //     .create_message(log_channel)
        //     .embeds(&[embed.build()])?
        //     .await?;

        Ok(())
    }
}
