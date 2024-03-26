use twilight_model::gateway::payload::incoming::{MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate};

use crate::message::Message;

#[derive(Debug)]
pub enum MessageSync<'a> {
    /// Created from an existing message
    Message(&'a twilight_model::channel::Message),
    /// Added / crafted manually
    Custom(&'a Message),
    /// Created from a message update event
    MessageUpdate(&'a MessageUpdate),
    /// Created from a message delete event
    MessageDelete(&'a MessageDelete),
    /// Created from a message delete bulk event
    MessageDeleteBulk(&'a MessageDeleteBulk),
    /// Created from a message create event
    MessageCreate(&'a MessageCreate),
}

impl<'a> From<&'a Message> for MessageSync<'a> {
    fn from(luro_message: &'a Message) -> Self {
        Self::Custom(luro_message)
    }
}

impl<'a> From<&'a twilight_model::channel::Message> for MessageSync<'a> {
    fn from(twilight_message: &'a twilight_model::channel::Message) -> Self {
        Self::Message(twilight_message)
    }
}
