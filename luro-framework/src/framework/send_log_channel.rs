use anyhow::anyhow;
use luro_model::{database::drivers::LuroDatabaseDriver, guild::log_channel::LuroLogChannel, response::LuroResponse};
use twilight_http::Response;
use twilight_model::{
    channel::Message,
    id::{marker::GuildMarker, Id}
};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Sends a message to the specified guild log channel. If the channel is not set, uses the catchall channel.
    ///
    /// If the catchall channel is not defined, then silently drop the log.
    pub async fn send_log_channel<F: FnOnce(&mut LuroResponse) -> &mut LuroResponse>(
        &self,
        guild_id: &Id<GuildMarker>,
        kind: LuroLogChannel,
        response: F
    ) -> anyhow::Result<Response<Message>> {
        let guild_data = self.database.get_guild(guild_id).await?;
        let log_channel = match kind {
            LuroLogChannel::Catchall => guild_data.catchall_log_channel,
            LuroLogChannel::Message => guild_data.message_events_log_channel,
            LuroLogChannel::Moderator => guild_data.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_data.thread_events_log_channel
        };

        let log_channel = match log_channel {
            Some(data) => data,
            None => match guild_data.catchall_log_channel {
                Some(channel) => channel,
                None => return Err(anyhow!("Guild {guild_id} does not have a catchall channel defined"))
            }
        };

        Ok(self.send_message(&log_channel, response).await?)
    }
}
