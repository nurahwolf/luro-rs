use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UserMarriage {
    /// The user's optional reason for marrying
    pub reason: String,
    /// Person who initiated the marriage
    pub proposer_id: Id<UserMarker>,
    /// Person who accepted the marriage
    pub proposee_id: Id<UserMarker>,
    /// Are they divorced
    pub divorced: bool,
    /// Was their marriage proposal rejected
    pub rejected: bool,
}
