use twilight_model::id::{marker::UserMarker, Id};

impl super::Guild {
    pub fn is_owner(&self, user_id: &Id<UserMarker>) -> bool {
        user_id == &self.owner_id
    }
}
