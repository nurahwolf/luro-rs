use twilight_model::id::{marker::ChannelMarker, Id};

use super::Message;

#[derive(Debug, Clone)]
pub struct Quote {
    pub channel_id: Id<ChannelMarker>,
    pub message: Message,
    pub nsfw: bool,
    pub quote_id: i64,
}
