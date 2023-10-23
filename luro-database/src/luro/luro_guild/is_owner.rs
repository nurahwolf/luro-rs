use twilight_model::id::{marker::UserMarker, Id};

use crate::LuroGuild;

impl LuroGuild {
    pub fn is_owner(&self, user_id: &Id<UserMarker>) -> bool {
        user_id == &self.owner_id
    }
}
