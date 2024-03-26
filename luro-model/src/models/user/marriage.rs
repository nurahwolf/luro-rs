use twilight_model::id::{marker::UserMarker, Id};

pub struct Marriage {
    /// Person who initiated the marriage
    pub proposer_id: Id<UserMarker>,
    /// Person who accepted the marriage
    pub proposee_id: Id<UserMarker>,
    /// Are they divorced
    pub divorced: bool,
    /// Was their marriage proposal rejected
    pub rejected: bool,
    /// What was the reason for marrying
    pub reason: String,
}
