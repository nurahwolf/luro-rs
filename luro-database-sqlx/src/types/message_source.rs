#[derive(Default, Debug, ::sqlx::Type)]
#[sqlx(type_name = "message_source", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DbMessageSource {
    /// Created from an existing message
    Message,
    /// Added / crafted manually
    Custom,
    /// Created from a cached message
    CachedMessage,
    /// Created from a message update event
    MessageUpdate,
    /// Created from a message delete event
    MessageDelete,
    /// Created from a message create event
    MessageCreate,
    /// No message :(
    #[default]
    None,
}