use twilight_model::id::{marker::UserMarker, Id};

use crate::DatabaseUser;

impl DatabaseUser {
    /// Return's a Twilight [Id<UserMarker>]
    pub fn user_id(&self) -> Id<UserMarker> {
        Id::new(self.user_id as u64)
    }
}