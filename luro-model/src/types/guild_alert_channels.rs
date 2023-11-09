use twilight_model::id::{marker::ChannelMarker, Id};

#[derive(Default)]
pub struct GuildAlertChannels {
    pub catchall_log_channel: Option<i64>,
    pub message_events_log_channel: Option<i64>,
    pub moderator_actions_log_channel: Option<i64>,
    pub thread_events_log_channel: Option<i64>,
}

impl GuildAlertChannels {
    pub fn catchall_log_channel(&self) -> Option<Id<ChannelMarker>> {
        self.catchall_log_channel.map(|x| Id::new(x as u64))
    }

    pub fn message_events_log_channel(&self) -> Option<Id<ChannelMarker>> {
        self.message_events_log_channel.map(|x| Id::new(x as u64))
    }

    pub fn moderator_actions_log_channel(&self) -> Option<Id<ChannelMarker>> {
        self.moderator_actions_log_channel.map(|x| Id::new(x as u64))
    }

    pub fn thread_events_log_channel(&self) -> Option<Id<ChannelMarker>> {
        self.thread_events_log_channel.map(|x| Id::new(x as u64))
    }
}
