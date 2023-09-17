use luro_model::{guild::log_channel::LuroLogChannel, response::LuroResponse};
use tracing::debug;
use twilight_model::channel::Message;

use crate::CommandInteraction;

impl<T> CommandInteraction<T> {
    /// Attempts to send to a log channel if it is present.
    /// Returns a message if a message was sent
    pub async fn send_log_channel<F: FnOnce(&mut LuroResponse) -> &mut LuroResponse>(
        &self,
        log_channel: LuroLogChannel,
        response: F
    ) -> anyhow::Result<Option<Message>> {
        debug!("Attempting to send to log channel");
        // TODO: Send event to main logging channel if not defined
        let (guild_data, guild_id) = match self.guild_id {
            Some(guild_id) => (self.database.get_guild(&guild_id).await?, guild_id),
            None => return Ok(None),
        };

        let log_channel_requested = match log_channel {
            LuroLogChannel::Catchall => guild_data.catchall_log_channel,
            LuroLogChannel::Message => guild_data.message_events_log_channel,
            LuroLogChannel::Moderator => guild_data.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_data.thread_events_log_channel,
        };

        let log_channel = match log_channel_requested {
            Some(log_channel) => log_channel,
            None => match guild_data.catchall_log_channel {
                Some(channel) => channel,
                None => {
                    debug!("Guild {guild_id} does not have a catchall channel defined");
                    return Ok(None);
                }
            },
        };

        Ok(Some(self.send_message(&log_channel, response).await?.model().await?))
    }
}
