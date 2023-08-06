use tracing::{debug, warn};
use twilight_model::id::{marker::GuildMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::models::LuroLogChannel;

use super::LuroFramework;

impl LuroFramework {
    /// Attempts to send to a log channel if it is present.
    pub async fn send_log_channel(
        &self,
        guild_id: &Option<Id<GuildMarker>>,
        embed: EmbedBuilder,
        log_channel: LuroLogChannel
    ) -> anyhow::Result<()> {
        debug!("Attempting to send to log channel");
        let guild_id = match guild_id {
            Some(data) => data,
            None => return Ok(())
        };
        let guild_data = match self.data_guild.get(guild_id) {
            Some(data) => data,
            None => return Ok(())
        };
        let log_channel = match log_channel {
            LuroLogChannel::Catchall => guild_data.catchall_log_channel,
            LuroLogChannel::Message => guild_data.message_events_log_channel,
            LuroLogChannel::Moderator => guild_data.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_data.thread_events_log_channel
        };

        let log_channel = match log_channel {
            Some(data) => data,
            None => match guild_data.catchall_log_channel {
                Some(channel) => channel,
                None => {
                    warn!("Guild {guild_id} does not have a catchall channel defined");
                    return Ok(());
                }
            }
        };

        self.twilight_client
            .create_message(log_channel)
            .embeds(&[embed.build()])
            .await?;

        debug!("Successfully sent to log channel");
        Ok(())
    }
}
