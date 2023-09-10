use luro_model::{database::drivers::LuroDatabaseDriver, guild::log_channel::LuroLogChannel, response::LuroResponse};
use tracing::debug;

use super::LuroSlash;

impl<D: LuroDatabaseDriver,> LuroSlash<D,> {
    /// Send a message to a log channel if defined
    /// This gets the guild ID from the interaction. Consider using the method on ['LuroFramework'] to define the channel you are sending to.
    pub async fn send_log_channel<F,>(&self, log_channel: LuroLogChannel, response: F,) -> anyhow::Result<(),>
    where
        F: FnOnce(&mut LuroResponse,) -> &mut LuroResponse,
    {
        debug!("Attempting to send to log channel");
        // TODO: Send event to main logging channel if not defined
        let (guild_data, guild_id,) = match self.interaction.guild_id {
            Some(guild_id,) => (self.framework.database.get_guild(&guild_id,).await?, guild_id,),
            None => return Ok((),),
        };

        let log_channel_requested = match log_channel {
            LuroLogChannel::Catchall => guild_data.catchall_log_channel,
            LuroLogChannel::Message => guild_data.message_events_log_channel,
            LuroLogChannel::Moderator => guild_data.moderator_actions_log_channel,
            LuroLogChannel::Thread => guild_data.thread_events_log_channel,
        };

        let log_channel = match log_channel_requested {
            Some(log_channel,) => log_channel,
            None => match guild_data.catchall_log_channel {
                Some(channel,) => channel,
                None => {
                    debug!("Guild {guild_id} does not have a catchall channel defined");
                    return Ok((),);
                }
            },
        };

        self.framework.send_message(&log_channel, response,).await?;
        Ok((),)
    }
}
