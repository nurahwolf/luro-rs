use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

use crate::PRIMARY_BOT_OWNER;

/// A story, which is simply a title and content both as strings
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Story {
    pub title: String,
    pub description: String,
    pub author: Id<UserMarker>,
}

impl Default for Story {
    fn default() -> Self {
        Self {
            title: Default::default(),
            description: Default::default(),
            // Defaults to the primary owner
            author: PRIMARY_BOT_OWNER,
        }
    }
}
