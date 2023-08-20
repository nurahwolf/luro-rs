use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

use crate::constants::PRIMARY_BOT_OWNER;

/// Simply a [HashMap] that is keyed to [usize], containing a [Heck]
pub type Hecks = HashMap<usize, Heck>;

/// A specific heck, used in [Hecks]. This contains the message itself, and the user ID of the author.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Heck {
    pub heck_message: String,
    pub author_id: Id<UserMarker>
}

impl Default for Heck {
    fn default() -> Self {
        Self {
            heck_message: Default::default(),
            author_id: PRIMARY_BOT_OWNER
        }
    }
}
