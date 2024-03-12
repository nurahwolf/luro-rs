use twilight_model::id::{marker::UserMarker, Id};

impl super::MemberContext {
    pub fn user_id(&self) -> Id<UserMarker> {
        self.user.user_id
    }
}
