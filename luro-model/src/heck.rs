use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

use crate::PRIMARY_BOT_OWNER;

/// Simply a [BTreeMap] that is keyed to [usize], containing a [Heck]
pub type Hecks = BTreeMap<usize, Heck>;

/// A specific heck, used in [Hecks]. This contains the message itself, and the user ID of the author.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Heck {
    pub author_id: Id<UserMarker>,
    pub heck_message: String,
    #[serde(default)]
    pub nsfw: bool
}

impl Default for Heck {
    fn default() -> Self {
        Self {
            heck_message: Default::default(),
            author_id: PRIMARY_BOT_OWNER,
            nsfw: false
        }
    }
}
