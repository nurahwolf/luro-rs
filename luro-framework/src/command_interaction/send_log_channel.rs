use luro_model::{guild::log_channel::LuroLogChannel, response::LuroResponse};
use tracing::debug;
use twilight_model::{channel::Message, id::Id};

use crate::{CommandInteraction, Luro};

impl CommandInteraction {
    /// Attempts to send to a log channel if it is present.
    /// Returns a message if a message was sent
    pub async fn send_log_channel<F: FnOnce(&mut LuroResponse) -> &mut LuroResponse>(
        &self,
        log_channel: LuroLogChannel,
        response: F,
    ) -> anyhow::Result<Option<Message>> {
        debug!("Attempting to send to log channel");
        let guild = match &self.guild {
            Some(guild) => guild,
            None => return Ok(None),
        };
        let guild_alert_channels = guild.alert_channels().await?;

        let log_channel_requested = match log_channel {
            LuroLogChannel::Catchall => guild_alert_channels.catchall_log_channel,
            LuroLogChannel::Message => guild_alert_channels.message_events_log_channel,
            LuroLogChannel::Moderator => guild_alert_channels.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_alert_channels.thread_events_log_channel,
        };

        let log_channel = match log_channel_requested {
            Some(log_channel) => log_channel,
            None => match guild_alert_channels.catchall_log_channel {
                Some(channel) => channel,
                None => {
                    debug!("Guild {} does not have a catchall channel defined", guild.guild_id);
                    return Ok(None);
                }
            },
        };

        Ok(Some(
            self.send_message(&Id::new(log_channel as u64), response).await?.model().await?,
        ))
    }
}
