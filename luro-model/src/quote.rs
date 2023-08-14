use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

use crate::constants::PRIMARY_BOT_OWNER;

/// A story, which is simply a title and content both as strings
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Quote {
    pub quote: String,
    pub author: Id<UserMarker>
}

impl Default for Quote {
    fn default() -> Self {
        Self {
            quote: Default::default(),
            // Defaults to the primary owner
            author: PRIMARY_BOT_OWNER
        }
    }
}
