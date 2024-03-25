use twilight_model::id::{marker::UserMarker, Id};

impl super::InteractionContext {
    pub fn author_id(&self) -> Id<UserMarker> {
        self.interaction.author_id().unwrap()
    }
}
