use twilight_model::id::{marker::UserMarker, Id};

use crate::LuroGuild;

impl LuroGuild {
    pub fn owner_id(&self) -> Id<UserMarker> {
        Id::new(self.owner_id as u64)
    }
}
